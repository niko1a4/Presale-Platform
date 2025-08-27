import * as anchor from "@coral-xyz/anchor";
import { Program, BN, web3 } from "@coral-xyz/anchor";
import { PresalePlatform } from "../target/types/presale_platform";
import { createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, getAccount, mintTo, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import { publicKey, token } from "@coral-xyz/anchor/dist/cjs/utils";
import { Connection, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID, associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
const { Keypair, LAMPORTS_PER_SOL } = web3;

describe("presale setup, init & deposit_tokens", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.PresalePlatform as Program<PresalePlatform>;

  //dev - presale authority and token owner (depositor)
  const dev = Keypair.generate();
  const buyer = Keypair.generate();

  let tokenMint: PublicKey;
  let presalePda: PublicKey;
  let presaleBump: number;

  let vaultPresaleAta: PublicKey;
  let vaultLpPda: PublicKey;
  let solVaultPda: PublicKey;
  let vaultLpBump: number;
  let solVaultBump: number;

  let devAta: PublicKey;
  let buyerAta: PublicKey;

  const DECIMALS = 6;

  const TARGET_PRESALE_TOKENS = new BN(100_000_000).mul(new BN(10).pow(new BN(DECIMALS))); //100M * 10^DECIMALS

  //deposit requirement -> target+(target/2)
  const DEPOSIT_TOTAL = TARGET_PRESALE_TOKENS.add(TARGET_PRESALE_TOKENS.div(new BN(2)));
  //150% of target

  const HARD_CAP_LAMPORTS = new BN(50 * LAMPORTS_PER_SOL);
  const SEED_U64 = new BN(69);
  const now = Math.floor(Date.now() / 1000);
  const END_TS = new BN(now + 3600);

  function seedLeU64(n: BN) {
    return Buffer.from(n.toArray("le", 8));
  }

  it("global setup: fund signers, create mint, derive PDAs and ATAs", async () => {
    for (const kp of [dev, buyer]) {
      const sig = await connection.requestAirdrop(kp.publicKey, 5 * LAMPORTS_PER_SOL);
      await connection.confirmTransaction(sig, "confirmed");
    }
    //mint controlled by dev:
    tokenMint = await createMint(
      connection,
      provider.wallet.payer,
      dev.publicKey,
      null,
      6,
    );
    const devTokenAcc = await getOrCreateAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      tokenMint,
      dev.publicKey,
    );
    //dev ATA where tokens for depositing will be minted to
    devAta = await getAssociatedTokenAddress(tokenMint, dev.publicKey);
    await mintTo(
      connection,
      provider.wallet.payer,
      tokenMint,
      devAta,
      dev,
      BigInt(DEPOSIT_TOTAL.toString()) //mint exactly what is deposited
    );

    //presale PDA
    [presalePda, presaleBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale"), dev.publicKey.toBuffer(), seedLeU64(SEED_U64)],
      program.programId,
    );

    //presale owned ATA , for presale tokens (allowOnwerOffCurve = true)
    vaultPresaleAta = await getAssociatedTokenAddress(tokenMint, presalePda, true);

    //pda token account for LP vault
    [vaultLpPda, vaultLpBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp-vault"), presalePda.toBuffer()],
      program.programId,
    );
    //PDA account for sol vault
    [solVaultPda, solVaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("sol-vault"), presalePda.toBuffer()],
      program.programId,
    );
    //buyerAta from claim_tokens later
    buyerAta = await getAssociatedTokenAddress(tokenMint, buyer.publicKey);
  });
  it("initialize_presale: creates presale + vaults + sets state", async () => {
    await program.methods.initialize(
      SEED_U64,
      HARD_CAP_LAMPORTS,
      TARGET_PRESALE_TOKENS,
      END_TS
    ).accounts({
      authority: dev.publicKey,
      tokenMint,
      presale: presalePda,
      tokenVaultPresale: vaultPresaleAta,
      tokenVaultLp: vaultLpPda,
      solVault: solVaultPda,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
      .signers([dev])
      .rpc();

    //fetch and assert state
    const presaleAcc: any = await program.account.presale.fetch(presalePda);

    expect(presaleAcc.bump).to.equal(presaleBump);
    expect(presaleAcc.authority.toBase58()).to.eq(dev.publicKey.toBase58());
    expect(presaleAcc.tokenMint.toBase58()).to.eq(tokenMint.toBase58());
    expect(presaleAcc.tokenVaultPresale.toBase58()).to.eq(vaultPresaleAta.toBase58());
    expect(presaleAcc.tokenVaultLp.toBase58()).to.eq(vaultLpPda.toBase58());
    expect(presaleAcc.solVault.toBase58()).to.eq(solVaultPda.toBase58());

    expect(presaleAcc.hardCapLamports.toString()).to.eq(HARD_CAP_LAMPORTS.toString());
    expect(presaleAcc.targetPresaleTokens.toString()).to.eq(
      TARGET_PRESALE_TOKENS.toString()
    );
    // start_time_unix set by program to current time
    const startTs = Number(presaleAcc.startTimeUnix);
    expect(startTs).to.be.a("number");

    expect(presaleAcc.endTimeUnix.toString()).to.eq(END_TS.toString());

    // default LP fields until finalize_presale
    expect(presaleAcc.lpTokenMint.toBase58()).to.eq(PublicKey.default.toBase58());
    expect(presaleAcc.lpTokenVault.toBase58()).to.eq(PublicKey.default.toBase58());

    // totals & flags
    expect(presaleAcc.tokensDepositedPresale.toNumber()).to.eq(0);
    expect(presaleAcc.tokensDepositedLp.toNumber()).to.eq(0);
    expect(presaleAcc.solRaisedLamports.toNumber()).to.eq(0);
    expect(presaleAcc.isFinalized).to.eq(false);
    expect(presaleAcc.isCanceled).to.eq(false);

    // also assert that the two vault accounts actually exist with right mint/owner
    // vaultPresaleAta = ATA(owner presalePda)
    const presaleVaultAcc = await getAccount(connection, vaultPresaleAta);
    expect(presaleVaultAcc.owner.toBase58()).to.eq(presalePda.toBase58());
    expect(presaleVaultAcc.mint.toBase58()).to.eq(tokenMint.toBase58());

    // vaultLpPda = PDA token account, owner = presalePda
    const lpVaultAcc = await getAccount(connection, vaultLpPda);
    expect(lpVaultAcc.owner.toBase58()).to.eq(presalePda.toBase58());
    expect(lpVaultAcc.mint.toBase58()).to.eq(tokenMint.toBase58());
  })
  it("deposit_tokens(): moves 100% target to presale ATA and 50% to lp pda vault", async () => {
    //balances before
    const devBefore = await getAccount(connection, devAta);
    const presaleVaultBefore = await getAccount(connection, vaultPresaleAta);
    const lpVaultBefore = await getAccount(connection, vaultLpPda);

    await program.methods.depositTokens(
      DEPOSIT_TOTAL
    ).accounts({
      depositor: dev.publicKey,
      tokenMint,
      presale: presalePda,
      tokenVaultPresale: vaultPresaleAta,
      tokenAta: devAta,
      tokenVaultLp: vaultLpPda,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    }).signers([dev])
      .rpc();

    //balances after
    const devAfter = await getAccount(connection, devAta);
    const presaleVaultAfter = await getAccount(connection, vaultPresaleAta);
    const lpVaultAfter = await getAccount(connection, vaultLpPda);

    const target = BigInt(TARGET_PRESALE_TOKENS.toString());
    const halfTarget = target / 2n;
    const total = BigInt(DEPOSIT_TOTAL.toString());
    // dev sent exactly total
    expect(devBefore.amount - devAfter.amount).to.eq(total);

    // presale vault got target
    expect(presaleVaultAfter.amount - presaleVaultBefore.amount).to.eq(target);

    // lp vault got target/2
    expect(lpVaultAfter.amount - lpVaultBefore.amount).to.eq(halfTarget);

    // state updated
    const presaleAcc: any = await program.account.presale.fetch(presalePda);
    expect(presaleAcc.tokensDepositedPresale.toString()).to.eq(
      TARGET_PRESALE_TOKENS.toString()
    );
    expect(presaleAcc.tokensDepositedLp.toString()).to.eq(
      TARGET_PRESALE_TOKENS.div(new BN(2)).toString()
    );
  });
});


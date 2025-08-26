# Presale Platform (Solana / Anchor)

A vault-style **fair launch / presale** program for SPL tokens on Solana, built with **Anchor**.

---

## Overview

The goal of this project is to make presales transparent and secure:

* The project team locks tokens in PDA-owned vaults (no human custody).
* Participants deposit SOL until a predefined hard cap.
* When the presale ends, the program finalizes: closing deposits, adding liquidity, and enabling token claims.
* Users then claim tokens proportional to their contribution.

---

## How It Works

1. **Initialize Presale**: Creates state and vaults for tokens and SOL.
2. **Deposit Tokens**: Project authority provides the token supply for presale and liquidity.
3. **Deposit SOL**: Participants contribute SOL until the hard cap is reached or the sale ends.
4. **Finalize Presale**: Authority finalizes the presale, which locks in the pricing and adds liquidity.
5. **Claim Tokens**: Participants claim tokens based on their share of total SOL contributed.

---

## Features

* PDA-owned vaults for both tokens and SOL
* Hard cap in lamports
* Fair, deterministic pricing based on total deposits

---

## Development

```bash
# Build
anchor build

# Test
anchor test

# Deploy (example)
solana config set --url https://api.devnet.solana.com
anchor deploy
```

---

## Notes

* Work in progress, code is unfinished yet.
* This code is experimental and unaudited. 

---


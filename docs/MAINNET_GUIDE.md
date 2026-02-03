# Mainnet Connection Guide

## Switching Jubilee Pocket from Devnet to Mainnet

This guide explains how to connect the mobile app to mainnet programs after deploying jSOLi and jUSDi.

---

## Overview

Currently, the app connects to **Devnet**. On mainnet launch (Feb 7, 2026):

1. Deploy Guardian Vault program to mainnet
2. Update program IDs in the app
3. Create liquidity pools on Orca
4. Update Jupiter swap configuration
5. Rebuild and release APK

---

## Step 1: Deploy Programs to Mainnet

### Guardian Vault Program
```bash
# Configure for mainnet
solana config set --url mainnet-beta

# Verify wallet has SOL (need ~2-5 SOL for deployment)
solana balance

# Deploy program
anchor deploy --provider.cluster mainnet
```

### Record New Program IDs
After deployment, note the mainnet program IDs:
- Guardian Vault: `<MAINNET_GUARDIAN_VAULT_ID>`
- jUSDi Mint: `<MAINNET_JUSDI_MINT>`
- SKR Mint: `<MAINNET_SKR_MINT>`

---

## Step 2: Update App Configuration

### Update `mobile/src/utils/solana.ts`

```typescript
// Change from:
export const CLUSTER = 'devnet';
export const RPC_ENDPOINT = 'https://api.devnet.solana.com';

// To:
export const CLUSTER = 'mainnet-beta';
export const RPC_ENDPOINT = 'https://api.mainnet-beta.solana.com';
// Or use a dedicated RPC (recommended for production):
// export const RPC_ENDPOINT = 'https://your-rpc-provider.com';
```

### Update Program IDs
Create or update `mobile/src/constants/programs.ts`:

```typescript
// Devnet IDs (for reference)
export const DEVNET = {
  GUARDIAN_VAULT: 'DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar',
  JUSDI_MINT: '<devnet_mint>',
  SKR_MINT: '<devnet_skr>',
};

// Mainnet IDs (update after deployment)
export const MAINNET = {
  GUARDIAN_VAULT: '<MAINNET_GUARDIAN_VAULT_ID>',
  JUSDI_MINT: '<MAINNET_JUSDI_MINT>',
  SKR_MINT: '<MAINNET_SKR_MINT>',
};

// Active configuration
export const PROGRAM_IDS = process.env.NETWORK === 'mainnet' ? MAINNET : DEVNET;
```

---

## Step 3: Create Orca Liquidity Pool

### jUSDi/USDC Pool

1. Go to [orca.so](https://www.orca.so/)
2. Connect your wallet (with jUSDi and USDC)
3. Navigate to **Pools** → **Create Pool**
4. Select:
   - Token A: `jUSDi` (your mainnet mint address)
   - Token B: `USDC` (EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v)
5. Set initial price: `1.00` (1 jUSDi = 1 USDC)
6. Add liquidity:
   - **Minimum**: $5,000 each side
   - **Recommended**: $25,000 each side
7. Confirm and sign transaction

### SOL/SKR Pool (if needed)
Same process for SOL/SKR pair.

---

## Step 4: Update Jupiter Configuration

After creating the pool, Jupiter will automatically detect it within ~1 hour.

### Verify in SwapScreen
The swap functionality will work once Jupiter indexes the pool:

```typescript
// In SwapScreen.tsx, Jupiter auto-detects pools
// No manual configuration needed - just verify the tokens are tradeable
```

### Test the swap:
1. Open Jubilee Pocket
2. Navigate to Swap
3. Enter small test amount
4. Verify quote appears

---

## Step 5: Rebuild APK for Mainnet

```bash
cd mobile/android

# Clean previous build
./gradlew clean

# Set environment to mainnet
export NETWORK=mainnet

# Build release APK
./gradlew assembleRelease

# APK location
ls app/build/outputs/apk/release/
```

---

## Step 6: Update Pyth Oracle Feeds

### Mainnet Pyth Feed IDs

| Asset | Mainnet Feed ID |
|-------|-----------------|
| SOL/USD | `H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG` |
| USDC/USD | `Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD` |

Update in your program's initialize instruction or config.

---

## Recommended RPC Providers

For production, use a dedicated RPC:

| Provider | URL |
|----------|-----|
| Helius | `https://mainnet.helius-rpc.com/?api-key=YOUR_KEY` |
| QuickNode | `https://your-endpoint.solana-mainnet.quiknode.pro/` |
| Alchemy | `https://solana-mainnet.g.alchemy.com/v2/YOUR_KEY` |
| Triton | `https://jubilee-pocket.rpcpool.com` |

---

## Checklist Before Launch

- [ ] Guardian Vault deployed to mainnet
- [ ] jUSDi mint created on mainnet
- [ ] Program IDs updated in app
- [ ] RPC endpoint changed to mainnet
- [ ] Orca pool created with sufficient liquidity
- [ ] Jupiter indexing verified
- [ ] Pyth feed IDs updated
- [ ] APK rebuilt for mainnet
- [ ] Tested deposit/borrow/swap flows

---

## Timeline

| Date | Action |
|------|--------|
| Feb 7 | Deploy jSOLi/jUSDi programs to mainnet |
| Feb 7 | Create Orca pools (~$10K+ liquidity) |
| Feb 8 | Verify Jupiter integration |
| Feb 8 | Release mainnet APK |

---

## Troubleshooting

### "Transaction failed"
- Ensure wallet has SOL for fees (~0.01 SOL minimum)
- Verify RPC endpoint is responsive

### "No quote available" in Swap
- Pool may not be indexed yet (wait 1-2 hours)
- Ensure pool has sufficient liquidity (>$1K)

### "Price unavailable"
- Check Pyth feed IDs are correct for mainnet
- Verify oracle account is valid

---

© 2026 Jubilee Labs LLC

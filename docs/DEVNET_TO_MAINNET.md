# Guardian Vault - Devnet to Mainnet Roadmap

## Pre-Devnet Requirements (Current Phase)

### ✅ Completed
- [x] All CRITICAL security issues fixed
- [x] All HIGH security issues fixed
- [x] Pyth SDK integration with devnet feature flag
- [x] jUSDi mint authority constraint
- [x] All tests passing (3/3)
- [x] Audit score: 93/100

### ⏳ Pending
- [ ] Acquire 3.5 SOL for Devnet deployment
- [ ] Deploy program binary
- [ ] Initialize VaultState PDA
- [ ] Create/link test token mints

---

## Mobile Testing Strategy

### A. Testing WITHOUT Solana Seeker (Emulator/Browser)

1. **Local Emulator Setup**
   ```bash
   # Start local validator
   solana-test-validator --reset
   
   # Deploy program
   anchor deploy
   
   # Run integration tests
   anchor test --skip-local-validator
   ```

2. **React Native Simulator**
   - Use Expo with Solana Web3.js
   - Mock the Seed Vault with a local keypair
   - Test UI flows: deposit, borrow, harvest, withdraw

3. **Devnet Web Testing**
   - Connect with Phantom/Backpack browser extension
   - Test all transactions on Devnet
   - Verify jUSDi minting and transfers

### B. Testing WITH Solana Seeker Device

1. **Prerequisites**
   - Solana Seeker with Seed Vault enabled
   - Mobile Wallet Adapter (MWA) SDK integrated
   - Devnet-configured app build

2. **Seed Vault Integration**
   ```typescript
   // SeedVaultService.ts
   import { transact } from '@solana-mobile/mobile-wallet-adapter-protocol';
   
   const signTx = async (tx: Transaction) => {
     return await transact(async (wallet) => {
       await wallet.authorize({ cluster: 'devnet' });
       const signed = await wallet.signTransactions({ transactions: [tx] });
       return signed[0];
     });
   };
   ```

3. **Test Cases**
   | Test | Steps | Expected |
   | :--- | :--- | :--- |
   | Connect Wallet | Open app → Tap "Connect" | Seed Vault prompt appears |
   | Biometric Auth | Tap "Deposit" → Approve | Face/Fingerprint triggers |
   | Deposit Flow | Enter amount → Confirm | SKR transferred, jUSDi minted |
   | Harvest | Wait 1 min → Tap Harvest | Debt reduced |
   | Withdraw | Zero debt → Withdraw | Cooldown starts → Complete after 48h |

---

## Devnet Deployment Costs

| Item | SOL | Notes |
| :--- | ---: | :--- |
| Program Deploy (~150KB) | ~3.0 | Binary upload |
| VaultState PDA | ~0.002 | 256 bytes |
| GuardianList PDA | ~0.003 | ~500 bytes |
| Test Transactions (200) | ~0.1 | At 5000 lamports/tx |
| Buffer | ~0.4 | For failed txs |
| **Total Devnet** | **~3.5 SOL** | Request from faucet |

---

## Mainnet Roadmap

### Phase 1: Pre-Mainnet (2-4 weeks)
| Task | Duration | Status |
| :--- | :---: | :---: |
| Devnet deployment | 1 day | ⏳ Pending |
| Full lifecycle testing | 3 days | ⏳ Pending |
| Mobile app integration | 1 week | ⏳ Pending |
| Community beta testing | 1 week | ⏳ Pending |
| Bug fixes | 1 week | ⏳ Pending |

### Phase 2: Security (2-3 weeks)
| Task | Duration | Cost |
| :--- | :---: | ---: |
| External audit (Neodyme/OtterSec) | 2 weeks | $15-50K |
| Audit remediation | 1 week | — |
| Re-audit (if needed) | 1 week | $5-10K |

### Phase 3: Mainnet Launch
| Task | Duration | Cost (SOL) |
| :--- | :---: | ---: |
| Program deployment | 1 day | ~10-15 |
| Initialize VaultState | — | ~0.01 |
| Transfer mint authority | — | ~0.001 |
| Initial transactions | — | ~0.5 |
| **Total Mainnet** | | **~12-16 SOL** |

---

## Mainnet Cost Breakdown

| Category | Cost (USD) | Notes |
| :--- | ---: | :--- |
| **Program Deployment** | $500-1,000 | At $50-70/SOL |
| **Account Initialization** | $1-5 | PDAs + ATAs |
| **External Audit** | $15,000-50,000 | Neodyme/OtterSec/Zellic |
| **Bug Bounty Fund** | $10,000+ | Optional but recommended |
| **Legal/Compliance** | $5,000+ | If applicable |
| **Total Estimated** | **$25K-70K** | Conservative range |

---

## Post-Mainnet Checklist

- [ ] Monitor program logs (Helius/Triton)
- [ ] Set up circuit breaker bot
- [ ] Transition to Squads multisig (2/3 or 3/5)
- [ ] Document incident response plan
- [ ] Enable Pyth price feed (remove devnet fallback)
- [ ] Bug bounty program (Immunefi)

---

*"For which of you, desiring to build a tower, does not first sit down and count the cost?"* — Luke 14:28

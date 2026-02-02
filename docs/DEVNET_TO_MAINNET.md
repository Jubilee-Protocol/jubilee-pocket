# Guardian Vault - Devnet to Mainnet Roadmap

> **Last Updated**: February 2, 2026
> **Current Phase**: Devnet Testing

---

## ✅ Phase 1: Pre-Devnet (COMPLETED)

| Task | Status |
| :--- | :---: |
| All CRITICAL security issues fixed | ✅ |
| All HIGH security issues fixed | ✅ |
| Pyth SDK integration | ✅ |
| jUSDi mint authority constraint | ✅ |
| All tests passing (3/3) | ✅ |
| Audit score: 93/100 | ✅ |
| **Devnet Deployment** | ✅ **LIVE** |

### Deployed Program
| Item | Value |
| :--- | :--- |
| **Program ID** | `DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar` |
| **Network** | Devnet |
| **Deploy Date** | Feb 2, 2026 |
| **Deploy Cost** | ~3.4 SOL |

---

## 🔄 Phase 2: Devnet Testing (CURRENT)

### Immediate Tasks
- [ ] Initialize VaultState PDA on Devnet
- [ ] Create test SKR mint
- [ ] Create jUSDi mint + transfer authority to vault
- [ ] Build mobile app (Xcode in progress)
- [ ] Test full lifecycle on Seeker device

### Mobile App Tasks
- [ ] Connect React Native to deployed program
- [ ] Implement MWA transact() with Seed Vault
- [ ] Build release APK
- [ ] Record demo video

### Testing Checklist
| Test | Status |
| :--- | :---: |
| Connect Seed Vault | ⏳ |
| Deposit SKR collateral | ⏳ |
| Borrow jUSDi | ⏳ |
| Harvest rewards | ⏳ |
| Start withdrawal cooldown | ⏳ |
| Complete withdrawal | ⏳ |
| Liquidation (if applicable) | ⏳ |

---

## ⏳ Phase 3: Hackathon Submission (This Week)

| Deliverable | Status |
| :--- | :---: |
| Functional APK | ⏳ Building |
| GitHub repo | ✅ [jubilee-pocket](https://github.com/Jubilee-Protocol/jubilee-pocket) |
| Demo video | ⏳ Record after Seeker test |
| Pitch deck | ⏳ Use outline in `hackathon/SUBMISSION.md` |

---

## 📋 Phase 4: Pre-Mainnet (2-4 weeks post-hackathon)

| Task | Duration | Status |
| :--- | :---: | :---: |
| Community beta testing | 1 week | ⏳ |
| Bug fixes from testing | 1 week | ⏳ |
| External security audit | 2-3 weeks | ⏳ |
| Audit remediation | 1 week | ⏳ |

### External Audit Options
| Auditor | Est. Cost | Timeline |
| :--- | ---: | :---: |
| [Neodyme](https://neodyme.io/) | $25-50K | 2-3 weeks |
| [OtterSec](https://osec.io/) | $30-60K | 2-4 weeks |
| [Zellic](https://zellic.io/) | $40-80K | 3-4 weeks |
| [sec3](https://sec3.dev/) | $15-30K | 1-2 weeks |

---

## 🚀 Phase 5: Mainnet Launch

### Deployment Costs
| Item | SOL | USD (at $50/SOL) |
| :--- | ---: | ---: |
| Program deployment (~500KB) | ~10-15 | $500-750 |
| VaultState PDA | ~0.01 | $0.50 |
| jUSDi mint creation | ~0.01 | $0.50 |
| Initial transactions | ~0.5 | $25 |
| **Total Deploy** | **~12-16** | **$600-800** |

### Pre-Launch Checklist
- [ ] Remove devnet feature flag (use real Pyth oracle)
- [ ] Update program ID in Anchor.toml
- [ ] Generate new keypair for mainnet
- [ ] Deploy with upgrade authority
- [ ] Initialize VaultState with mainnet params
- [ ] Transfer jUSDi mint authority to vault PDA
- [ ] Set up Squads multisig (2/3 or 3/5)

### Total Mainnet Costs (Conservative)
| Category | Cost (USD) |
| :--- | ---: |
| Program Deployment | $600-800 |
| External Audit | $15,000-50,000 |
| Bug Bounty Fund | $10,000+ |
| Legal/Compliance | $5,000+ |
| **Total** | **$30K-70K** |

---

## 📊 Post-Mainnet Operations

- [ ] Monitor with Helius/Triton RPCs
- [ ] Set up circuit breaker bot (auto-pause on anomalies)
- [ ] Document incident response plan
- [ ] Launch bug bounty (Immunefi)
- [ ] Gradual TVL increase with deposit caps

---

## Quick Commands

### Remove Devnet Feature (For Mainnet)
```bash
# Build WITHOUT devnet flag (uses real Pyth oracle)
cd programs/guardian-vault
cargo build-sbf
```

### Deploy to Mainnet
```bash
solana config set --url mainnet-beta
solana program deploy target/deploy/guardian_vault.so
```

### Initialize Vault on Mainnet
```bash
# Run initialization script (TBD)
anchor run init-mainnet
```

---

*"For which of you, desiring to build a tower, does not first sit down and count the cost?"* — Luke 14:28

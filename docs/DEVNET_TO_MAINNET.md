# Guardian Vault - Devnet to Mainnet Roadmap

> **Last Updated**: February 2, 2026
> **Current Phase**: Devnet Testing
> **Audit Status**: Jubilee Labs Internal Audit (93/100) ✅

---

## ✅ Phase 1: Development (COMPLETED)

| Task | Status |
| :--- | :---: |
| All CRITICAL security issues fixed | ✅ |
| All HIGH security issues fixed | ✅ |
| Pyth SDK integration | ✅ |
| jUSDi mint authority constraint | ✅ |
| All tests passing (3/3) | ✅ |
| **Jubilee Labs Internal Audit: 93/100** | ✅ |
| **Devnet Deployment** | ✅ **LIVE** |

### Deployed Program
| Item | Value |
| :--- | :--- |
| **Program ID** | `DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar` |
| **Network** | Devnet |
| **Deploy Date** | Feb 2, 2026 |

---

## 🔄 Phase 2: Devnet Testing (CURRENT)

- [ ] Initialize VaultState PDA on Devnet
- [ ] Create test SKR + jUSDi mints
- [ ] Build mobile app
- [ ] Test full lifecycle on Seeker device
- [ ] Record demo video

---

## ⏳ Phase 3: Hackathon Submission (This Week)

| Deliverable | Status |
| :--- | :---: |
| Functional APK | ⏳ |
| GitHub repo | ✅ |
| Demo video | ⏳ |
| Pitch deck | ⏳ |

---

## 🚀 Phase 4: Mainnet Beta Launch

> **Strategy**: Launch as public beta with Jubilee Labs internal audit (93/100). No external audit required for beta phase.

### Pre-Launch Checklist
- [ ] Complete Devnet testing
- [ ] Remove devnet feature flag (use real Pyth oracle)
- [ ] Generate mainnet keypair
- [ ] Deploy program
- [ ] Initialize VaultState with mainnet params
- [ ] Transfer jUSDi mint authority to vault PDA
- [ ] Set up Squads multisig (2/3)

### Mainnet Deployment Costs
| Item | SOL | USD (at $50/SOL) |
| :--- | ---: | ---: |
| Program deployment | ~10-15 | $500-750 |
| VaultState PDA | ~0.01 | $0.50 |
| jUSDi mint | ~0.01 | $0.50 |
| Transactions | ~0.5 | $25 |
| **Total** | **~12-16** | **$600-800** |

---

## 📊 Post-Mainnet Beta

- [ ] Monitor with Helius/Triton RPCs
- [ ] Set up circuit breaker bot
- [ ] Gradual TVL increase with deposit caps
- [ ] Community feedback collection
- [ ] Iterate based on usage

---

## Quick Commands

### Build for Mainnet (Remove Devnet Flag)
```bash
cd programs/guardian-vault
cargo build-sbf  # Without --features devnet
```

### Deploy to Mainnet
```bash
solana config set --url mainnet-beta
solana program deploy target/deploy/guardian_vault.so
```

---

*"For which of you, desiring to build a tower, does not first sit down and count the cost?"* — Luke 14:28

# Jubilee Pocket 📱

> **Guardian Vault Protocol — Collateralized Lending for Solana Seeker**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-Devnet-green.svg)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.28.0-blueviolet.svg)](https://www.anchor-lang.com/)

---

## Overview

Jubilee Pocket is a mobile-first lending protocol designed for the **Solana Seeker** device. Users deposit **Stakehouse Keeper Receipts (SKR)** as collateral and borrow **jUSDi** (Jubilee USD Index) against their holdings with up to **55% LTV** — plus a holder bonus for larger positions.

### Key Features

- 🏦 **Collateralized Lending** — Deposit SKR, borrow jUSDi
- 📈 **Yield Harvesting** — Automatically repays debt with staking rewards
- 🔐 **Seed Vault Integration** — Native hardware security on Seeker
- ⚡ **Pyth Oracle** — Real-time price feeds for accurate LTV
- 🛑 **Emergency Pause** — Circuit breaker for admin control
- 📱 **React Native App** — Mobile-first experience

---

## Program Addresses

### Devnet (Pending Deployment)
| Program | Address |
| :--- | :--- |
| Guardian Vault | `wy7kkPnizRCbXvrG6fBkuat6q8AwbwTgnjxhZWcg3Si` |
| Mock SKR Mint | TBD |
| jUSDi Mint | TBD |

---

## Repository Structure

```
jubilee-pocket/
├── programs/
│   └── guardian-vault/     # Anchor program
│       └── src/
│           ├── instructions/
│           ├── state/
│           └── errors.rs
├── mobile/                  # React Native app
├── tests/                   # Integration tests
├── docs/
│   ├── AUDIT_REPORT.md
│   ├── RED_TEAM_AUDIT.md
│   ├── INTEGRATION_GUIDE.md
│   └── DEVNET_TO_MAINNET.md
└── Anchor.toml
```

---

## Quick Start

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor 0.28.0
- Node.js 18+

### Build & Test

```bash
# Clone
git clone git@github.com:Jubilee-Protocol/jubilee-pocket.git
cd jubilee-pocket

# Install dependencies
yarn install

# Build program
anchor build

# Run tests
anchor test
```

### Deploy to Devnet

```bash
# Configure for devnet
solana config set --url devnet

# Get devnet SOL
solana airdrop 5

# Deploy
anchor deploy --provider.cluster devnet
```

---

## How It Works

### Deposit & Borrow Flow
1. User deposits SKR collateral
2. Pyth oracle provides real-time price
3. System calculates max borrow (55% LTV base)
4. jUSDi minted to user wallet

### Harvest & Repay
1. User calls `harvest_repay` instruction
2. Rewards calculated from staking APY
3. Debt reduced by reward amount (minus 3% fee)

### Withdrawal Flow (Two-Step)
1. **Start Cooldown** — User initiates withdrawal
2. **Wait 48 Hours** — Security cooldown
3. **Complete** — Collateral returned, account closed

---

## Security

| Audit | Score | Status |
| :--- | :---: | :---: |
| Internal Review | 93/100 | ✅ Complete |
| External Audit | — | ⏳ Pending |

See [AUDIT_REPORT.md](./docs/AUDIT_REPORT.md) for details.

---

## Roadmap

- [x] Core program development
- [x] Security audit (93/100)
- [x] Pyth oracle integration
- [ ] Devnet deployment
- [ ] Mobile app integration
- [ ] External security audit
- [ ] Mainnet launch

---

## Built By

**Jubilee Protocol** — Building the future of decentralized finance.

- [Website](https://jubilee.fi)
- [Twitter](https://twitter.com/JubileeProtocol)
- [Discord](https://discord.gg/jubilee)

---

## License

MIT License — see [LICENSE](./LICENSE)

---

*"The prudent see danger and take refuge, but the simple keep going and pay the penalty."* — Proverbs 22:3

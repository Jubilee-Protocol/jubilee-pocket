# Solana Mobile Hackathon 2026 Submission

> **Team**: Jubilee Labs  
> **Project**: Jubilee Pocket  
> **Category**: Mobile dApp  
> **Submitted**: February 2026

---

## 📱 Project Overview

**Jubilee Pocket** is a mobile-first self-repaying lending protocol for Solana Seeker. Users deposit SKR tokens as collateral and borrow jUSDi stablecoins — with yield automatically paying down debt over time.

### Key Innovation
First SKR-collateralized CDP (Collateralized Debt Position) on Solana with:
- 🔐 Seed Vault hardware security integration
- 📈 Auto-repaying loans via yield harvesting
- 🔄 Jupiter-powered token swaps
- 📲 Push notifications for LTV alerts

---

## ✅ Submission Checklist

| Requirement | Status | Details |
|-------------|--------|---------|
| **Android APK** | ✅ Ready | `cd mobile/android && ./gradlew assembleRelease` |
| **GitHub Repo** | ✅ Live | [Jubilee-Protocol/jubilee-pocket](https://github.com/Jubilee-Protocol/jubilee-pocket) |
| **Demo Video** | ⏳ Record | See script below |
| **Pitch Deck** | ⏳ Create | `hackathon/pitch-deck.pdf` |

---

## 📊 Judging Criteria Alignment

### 🔥 Stickiness & PMF (25%)
- **Daily Engagement**: Harvest rewards button encourages daily check-ins
- **Push Notifications**: LTV warnings and harvest completion alerts
- **SKR Holder Utility**: Unlock liquidity from staked positions

### 🎨 User Experience (25%)
- **Native Mobile Design**: Built with React Native from ground up
- **Jubilee Branding**: Consistent pink theme with custom icons/emojis
- **Demo Mode**: Test flows without connecting a wallet
- **Scroll-optimized**: All screens properly scrollable

### 💡 Innovation / X-Factor (25%)
- **First of its Kind**: No other SKR-backed lending on Solana
- **Self-Repaying**: Yield automatically reduces debt
- **Jupiter Integration**: Instant jUSDi → USDC swaps
- **Pyth Oracle**: Real-time price feeds for accurate LTV

### 🎤 Presentation & Demo (25%)
- **Clear Value Prop**: "Your Seeker pays for itself"
- **Polished Video**: See demo script below
- **Complete Submission**: All materials included

---

## 🛠️ Technical Stack

| Component | Technology |
|-----------|------------|
| Mobile App | React Native 0.73 |
| Wallet | Mobile Wallet Adapter (MWA) |
| Smart Contract | Anchor 0.28.0 |
| Oracle | Pyth Network |
| Swaps | Jupiter Aggregator API |
| Network | Solana Devnet (live) |

### Program Addresses (Devnet)
```
Guardian Vault: DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar
```

---

## 🎬 Demo Video Script (2-3 min)

### Scene 1: Introduction (20s)
- Show Jubilee Pocket app icon on home screen
- "Jubilee Pocket — self-repaying loans for Solana Seeker"

### Scene 2: Connect Wallet (20s)
- Open app → Tap "Connect Wallet"
- Show Seed Vault authorization
- Or demo mode for testing

### Scene 3: Deposit Collateral (30s)
- Navigate to Deposit screen
- Enter SKR amount
- Show transaction signing with MWA
- Collateral appears in VaultCard

### Scene 4: Borrow jUSDi (30s)
- Navigate to Borrow screen
- Show LTV slider (55% max)
- Review transaction preview
- jUSDi minted to wallet

### Scene 5: Swap jUSDi (20s)
- Navigate to Swap screen
- Swap jUSDi → USDC
- Show Jupiter quote with price impact

### Scene 6: Harvest & Repay (20s)
- Show yield accumulation
- Tap "Harvest" to auto-repay
- Debt reduced by harvest amount

### Scene 7: Notifications (10s)
- Go to Settings → Test notifications
- Show harvest and LTV alerts

### Scene 8: Conclusion (10s)
- "Jubilee Pocket — making your Seeker pay for itself"
- Show GitHub repo

---

## 📁 Files Included

```
hackathon/
├── SUBMISSION.md       # This file
├── TOOLBOX.md          # SDK/libraries used
└── pitch-deck.pdf      # Presentation (to create)

mobile/
├── android/app/build/outputs/apk/release/
│   └── app-release.apk # Built APK
└── src/
    ├── screens/        # All UI screens
    ├── components/     # Reusable components
    └── services/       # NotificationService
```

---

## 🚀 Build Commands

```bash
# Build Android APK
cd mobile/android
./gradlew assembleRelease

# APK Location
ls mobile/android/app/build/outputs/apk/release/

# Install on Seeker
adb install mobile/android/app/build/outputs/apk/release/app-release.apk
```

---

## 👥 Team

**Jubilee Labs** — Building the Liberty Layer

---

## 📞 Contact

- Website: [jubileelabs.xyz](https://jubileelabs.xyz)
- Twitter: [@JubileeProtocol](https://twitter.com/JubileeProtocol)
- Telegram: [t.me/JubileeProtocol](https://t.me/JubileeProtocol)

---

*"The prudent see danger and take refuge, but the simple keep going and pay the penalty."* — Proverbs 22:3

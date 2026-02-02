# MONOLITH Hackathon Toolbox Resources

> Resources from the Solana Mobile hackathon toolbox for building Jubilee Pocket

---

## 🚀 Quick Start Templates

### Create New Project (Alternative to our existing setup)
```bash
npm create solana-dapp@latest
# Select "Solana Mobile" framework
```

---

## 📱 Mobile Wallet Adapter (MWA)

### Installation
We already have MWA installed in `mobile/package.json`:
```json
"@solana-mobile/mobile-wallet-adapter-protocol": "^2.2.5",
"@solana-mobile/mobile-wallet-adapter-protocol-web3js": "^2.2.5"
```

### ⚠️ Important: MWA is Android-Only
MWA is **not compatible with iOS** — Seeker is Android-based, so this is fine for us.

### Supported Wallets
- Seed Vault (Seeker built-in) ✅
- Phantom
- Solflare
- Ultimate Wallet

### Documentation
- [React Native MWA Guide](https://docs.solanamobile.com/react-native/wallet_ui_mobile_wallet_adapter)
- [Installation Details](https://docs.solanamobile.com/mobile-wallet-adapter/mobile-apps)

---

## 📚 Sample Apps to Reference

| App | Description | Use For |
| :--- | :--- | :--- |
| [Anchor Counter](https://github.com/solana-mobile/tutorial-apps/tree/main/AnchorCounterDapp) | IDL + Anchor program | **Our main reference** |
| [Idle Farming Game](https://github.com/solana-mobile/tutorial-apps/tree/main/FarmingIdleGame) | On-chain game | Gamification ideas |
| [Mobile NFT Minter](https://github.com/solana-mobile/tutorial-apps/tree/main/MobileNFTMinter) | Metaplex + IPFS | NFT receipt minting |
| [Settle](https://github.com/solana-mobile/react-native-samples/tree/main/settle) | P2P transactions | User flow patterns |
| [skr-address-resolution](https://github.com/solana-mobile/react-native-samples/tree/main/skr-address-resolution) | SKR domain lookup | **SKR integration** |
| [cause-pots](https://github.com/solana-mobile/react-native-samples/tree/main/cause-pots) | Anchor smart contract | Contract integration |
| [MintyFresh](https://github.com/solana-mobile/Minty-fresh/tree/main) | Production dApp | Full production patterns |

### 🎯 Most Relevant for Jubilee Pocket:
1. **Anchor Counter** — Anchor IDL integration with React Native
2. **skr-address-resolution** — SKR-specific functionality
3. **cause-pots** — Similar Anchor contract integration pattern

---

## 🤖 AI Toolkit (Claude Skills)

### Solana Mobile Skills
```bash
npx skills add https://github.com/wallet-ui/wallet-ui --skill install-wallet-ui-react-native
```

### Expo Skills
```bash
npx skills add https://github.com/expo/skills
```

### Solana Dev Skills
```bash
npx skills add https://github.com/solana-foundation/solana-dev-skill
```

---

## 📖 Documentation Links

### Solana Mobile
| Doc | URL |
| :--- | :--- |
| React Native Quickstart | https://docs.solanamobile.com/react-native/quickstart |
| MWA for Mobile Apps | https://docs.solanamobile.com/mobile-wallet-adapter/mobile-apps |
| Sample Apps | https://docs.solanamobile.com/sample-apps/sample_app_overview |
| AI Toolkit | https://docs.solanamobile.com/developers/ai-toolkit |
| Environment Setup | https://docs.solanamobile.com/react-native/setup |

### General Solana
| Doc | URL |
| :--- | :--- |
| Solana Docs | https://solana.com/docs |
| Expo Docs | https://docs.expo.dev/ |

---

## 🔧 Development Commands

### Build Android APK
```bash
cd mobile/android
./gradlew assembleRelease
```

### Run on Seeker
```bash
# Connect Seeker via USB
adb devices

# Install APK
adb install app/build/outputs/apk/release/app-release.apk
```

### Start Metro
```bash
cd mobile
npx react-native start
```

---

## 📋 Integration Checklist

- [x] MWA SDK installed
- [x] @solana/web3.js installed
- [x] React Native project setup
- [ ] Connect to Guardian Vault program
- [ ] Implement transact() for Seed Vault
- [ ] Build release APK
- [ ] Test on Seeker

---

*Reference these resources when implementing MWA and Seed Vault integration.*

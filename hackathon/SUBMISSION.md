# MONOLITH Hackathon Submission

> **Team**: Jubilee Protocol
> **Project**: Jubilee Pocket
> **Category**: Mobile
> **Deadline**: [TBD]

---

## Submission Requirements

### 1. The APK ⏳
- [ ] Build release APK: `cd mobile/android && ./gradlew assembleRelease`
- [ ] Test on Seeker device
- [ ] Upload to submission portal

### 2. GitHub Repo ✅
- [x] **Live**: https://github.com/Jubilee-Protocol/jubilee-pocket
- [x] Source code committed
- [x] README with documentation
- [x] MIT License

### 3. Demo Video ⏳
- [ ] Record full user flow (2-3 min)
- [ ] Show: Connect → Deposit → Borrow → Harvest → Withdraw
- [ ] Highlight Seed Vault integration
- [ ] Upload to YouTube/Loom

### 4. Pitch Deck ⏳
- [ ] Create slides (see outline below)
- [ ] Export to PDF

---

## Judging Criteria

| Criteria | Weight | Our Approach |
| :--- | :---: | :--- |
| **Stickiness & PMF** | 25% | Daily harvest rewards, SKR holder utility |
| **User Experience** | 25% | Mobile-native, biometric auth, clean UI |
| **Innovation/X Factor** | 25% | First SKR-collateralized CDP on Solana |
| **Presentation & Demo** | 25% | Polished video, clear value prop |

> *"Do not forsake the pretty things."*

---

## Technical Requirements Checklist

- [x] **Functional APK** — React Native app ready to build
- [x] **Solana Mobile Stack** — MWA protocol integrated
- [x] **Mobile Wallet Adapter** — @solana-mobile packages installed
- [x] **Built for Mobile** — Native React Native (not PWA/port)
- [x] **Meaningful Solana Interaction** — On-chain CDP with Pyth oracle

---

## Pitch Deck Outline

### Slide 1: Title
> **Jubilee Pocket**
> *Collateralized Lending for Solana Seeker*

### Slide 2: Problem
- SKR holders have locked value
- No native mobile lending on Seeker
- DeFi UX is intimidating on mobile

### Slide 3: Solution
- Deposit SKR → Borrow jUSDi (55% LTV)
- Yield auto-repays debt
- Seed Vault = hardware-grade security

### Slide 4: How It Works
```
[Deposit SKR] → [Pyth Price] → [Mint jUSDi] → [Harvest Rewards] → [Repay Debt]
```

### Slide 5: Demo
*[Insert demo video or GIF]*

### Slide 6: Stickiness
- Daily reward harvesting
- Debt management gamification
- Push notifications for LTV alerts

### Slide 7: Technical Architecture
```
Mobile App (React Native)
    ↓
Mobile Wallet Adapter
    ↓
Seed Vault (Seeker)
    ↓
Guardian Vault (Solana Program)
    ↓
Pyth Oracle
```

### Slide 8: Program Details
| Metric | Value |
| :--- | :--- |
| Program ID | `DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar` |
| Network | Devnet (Live) |
| Audit Score | 93/100 |
| LTV | 55% base, 60% SKR holders |

### Slide 9: Why Seeker?
- Seed Vault = institutional-grade custody
- On-device biometrics
- dApp Store native distribution
- Target audience: SKR holders

### Slide 10: Roadmap
- [x] Smart contract (93/100 audit)
- [x] Devnet deployment
- [ ] Hackathon demo
- [ ] External audit
- [ ] Mainnet launch

### Slide 11: Team
**Jubilee Protocol**
- Building the future of on-chain finance

### Slide 12: Ask
- Feedback from Solana Mobile team
- Integration support for dApp Store
- Community testing

---

## Files to Submit

| File | Status | Location |
| :--- | :---: | :--- |
| `jubilee-pocket.apk` | ⏳ | `mobile/android/app/build/outputs/apk/release/` |
| GitHub URL | ✅ | https://github.com/Jubilee-Protocol/jubilee-pocket |
| Demo Video | ⏳ | YouTube/Loom link |
| Pitch Deck | ⏳ | `hackathon/pitch-deck.pdf` |

---

## Quick Commands

```bash
# Build release APK
cd mobile/android
./gradlew assembleRelease

# APK location
ls -la app/build/outputs/apk/release/

# Install on Seeker
adb install app/build/outputs/apk/release/app-release.apk
```

---

*"The prudent see danger and take refuge, but the simple keep going and pay the penalty."* — Proverbs 22:3

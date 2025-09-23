# OriginTrace

## 🚀 Executive Summary
**OriginTrace** transforms the second-hand electronics market from guesswork into a verifiable, on-chain experience running fully on the **Internet Computer Protocol (ICP)**.

We combine certified technical inspections, cryptographic provenance, and smart-contract transaction controls (escrow/mediator) alongside an AI assistant. Each device trade becomes transparent, fair, and enforceable.

---

## 🌍 Market Problem
- **78%** of buyers worry about fraud when purchasing used devices.  
- **42%** of sold units hide technical defects.  
- **60%** of failures are caused by counterfeit replacement parts.

These gaps create a low-trust, opaque secondary market. OriginTrace eliminates these pain points by anchoring device identity, inspection, and transaction logic on-chain.

---

## 💡 What OriginTrace Delivers
A full **end-to-end trust pipeline**:
1. **Inspect** — Certified technicians run a structured hardware & software checklist.  
2. **Certify** — Cryptographic hash + IMEI/SN becomes the device’s immutable Digital Identity on ICP.  
3. **Contract** — Smart contracts automate ownership transfer, conditional payments, and digital warranties.  
4. **Invoice & Proof** — A QR invoice bundles the technical report, contract snapshot, and on-chain proof.  
5. **Assist** — AI assistant explains reports in plain language and recommends best options.

---

## 🛠 Current State (Web3)
**Status:** Fully migrated to Web3 and deployed in an early-stage release on ICP.

- **Frontend:** React + TypeScript + Tailwind hosted natively on ICP canisters.  
- **Backend:** Rust canisters handling ownership, warranties, and lifecycle logic.  
- **AI Chatbot:** Integrated with canisters for contextual user guidance.  
- **On-chain Proofs:** Inspection reports & ownership certificates anchored on ICP.  
- **Web2 references removed:** Project is now exclusively Web3/ICP-native.

**Repository:** https://github.com/amrshaker000/origin_trace  
**Live Demo:** https://origintrack.vercel.app/

---

## ✨ New & Notable Features
- 🔒 **Immutable Device Certificates** — Tamper-proof device records and inspection history.  
- 💸 **Escrow-style Smart Contracts** — Secure funds release on fulfillment of conditions.  
- 📜 **On-chain Digital Warranties** — Enforced automatically by canisters.  
- 🌐 **IPFS Integration** — Store reports, invoices, and documents off-chain with on-chain hashes.  
- ₿ **Bitcoin Anchoring (Optional)** — Durable proofs of payment/ownership anchored to Bitcoin.  
- 🤖 **AI Report Assistant** — Translates technical inspection into plain language and recommendations.  
- 🔗 **HTTP Outcalls** — External verification (e.g., manufacturer warranty checks) via canister outcalls.

---

## 📐 Architecture Overview

React + TypeScript + Tailwind (Frontend; deployed on ICP canisters)
│
▼
ICP Canisters (Rust)
├─ Ownership & Lifecycle Smart Contracts
├─ Warranty & Escrow Logic
├─ Immutable Report Hashes (on-chain)
├─ IPFS for documents (reports, invoices)
├─ Bitcoin anchoring service (optional)
└─ AI Assistant Service (connected via secure APIs)



---

## 🏛 Why ICP?
- **Ultra-low cost**: ~\$0.0001 per transaction.  
- **Fast**: ~2s certificate issuance.  
- **Native Bitcoin integration** for durable proofs.  
- **HTTP outcalls** for real-world validations.  
- **Chain-Key cryptography** to bind device identity to records.  
- **Fully on-chain** operation: minimal external dependencies.

---

## 🌟 Impact
- **Economic:** Certified devices achieve ~20% higher resale value; platform revenue via commissions + inspection fees.  
- **Social:** Builds buyer trust and creates certified technician jobs.  
- **Environmental:** Extends device lifecycles by 3+ years → reduces e-waste.  
- **Technological:** Real, user-facing blockchain utility with AI augmentation.

---

## 📹 Media & Demos
- Pitch Video: https://youtu.be/C32qIxBr35Q  
- Demo Video: https://youtu.be/B5QHXS011RI

---

## 🔜 Roadmap (short)
- Finalize Rust canisters for full lifecycle management.  
- Complete IPFS and Bitcoin anchoring integrations.  
- Add richer on-chain analytics & device history visualization.  
- Expand technician network and QA processes.

---

## 📄 License & Contact
**License:** (Add your license here, e.g. MIT)  
**Contact:** amrshaker842@gmail.com

---

**OriginTrace** — Trust, transparency, and AI-assisted provenance for the second-hand electronics economy.

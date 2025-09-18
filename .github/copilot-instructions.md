## Production/Live Mode Implementation

To transition UnifiedNUN from MVP/demo to a real, live blockchain platform, follow these steps:

- **Replace all mock/demo logic** with real blockchain, wallet, and smart contract integrations.
- **Integrate a real wallet provider** (e.g., MetaMask, WalletConnect, or a custom browser wallet) for user claims and transfers.
- **Implement persistent storage** (database or decentralized storage) for mined NUN, user sessions, and claims.
- **Deploy and connect real WASM/Rust smart contracts** for NUN minting, distribution, and claim logic. Audit contracts for security and efficiency.
- **Harden backend and frontend** for security, scalability, and reliability (input validation, error handling, rate limiting, etc.).
- **Deploy backend and frontend** to production infrastructure (cloud, CDN, HTTPS, etc.).
- **Add monitoring, logging, and alerting** for all critical services.
- **Document all production workflows** and update onboarding guides for users and developers.

**Key files to update:**
- `client/pages/dashboard.js`, `client/components/MiningCounter.js` (wallet integration, real claim logic)
- `backend/node/src/main.rs`, `contracts/wasm/` (real blockchain and contract logic)
- `backend/node/` (persistent storage, session management)

**Security:**
- Enforce all cryptographic operations (AES-256, RSA-4096, ZK proofs) in production.
- Regularly audit smart contracts and backend code.

**Go Live Checklist:**
- All endpoints and contracts use real data and wallets
- No hardcoded/demo values remain
- End-to-end tests pass in production environment

---

# Copilot Instructions for UnifiedNUN Blockchain

## Project Overview

UnifiedNUN is a next-generation blockchain platform featuring:
- **Universal Action Mining**: Every user or visitor action (visit, scroll, click, open, close, create, etc.) automatically mines a new NUN (block/unit). Each mined NUN is split: 50% to the user, 50% to the platform. Mining starts instantly and requires no setup.
- **Proof-of-Action Consensus**: AI-powered, engagement-optimized, and energy-efficient.
- **Browser-Based Nodes**: Nodes run in browsers, require <50MB storage, and sync in ~30 seconds.
- **WASM + Rust Smart Contracts**: All smart contracts are executed in a WebAssembly runtime for security and speed.
- **Gas-Free Transactions**: No transaction fees; NUN tokens are auto-minted via mining activities.


## Architecture & Patterns
- **Five-layer architecture**: Reference the technical architecture diagram (if available) for system boundaries.
- **Node Design**: Each browser node handles local processing (1000+ TPS) and network throughput (100+ TPS).
- **Smart Contracts**: Written in Rust, compiled to WASM, and sandboxed for safety.
- **AI Integration**: Reward distribution and consensus are managed by AI models optimizing for engagement and fairness.
- **Global Action Mining UX**: Implement a persistent, global mining counter and notification on every page. This counter displays a short explanation:  
	> "Every action you take (visit, scroll, click, open, close, create, etc.) automatically mines NUN for you. 50% goes to you, 50% to the platform. Claim your mined NUNs anytime!"
	The counter must be visible everywhere, with a global **Claim Your Mined NUN** button that leads to the dashboard for account creation or wallet connection to claim rewards.


## Developer Workflows
- **Build**: Use Rust and WASM toolchains for smart contract development. (e.g., `wasm-pack build`)
- **Test**: Automated tests should cover consensus, mining triggers, and contract execution. Use provided scripts or CI pipelines.
- **Debug**: Browser-based debugging tools are recommended for node/client issues.
- **Deployment**: CI/CD pipelines should be used for all deployments. Rollback procedures must be in place.
- **Action Mining Handler**: Implement a global event handler (JS/TS) to listen for user actions and trigger mining logic. Ensure the mining counter, notice, and claim button are always present and functional.

## Security & Conventions
- **Encryption**: All sensitive data uses AES-256 + RSA-4096 hybrid encryption.
- **Formal Verification**: Smart contracts should be formally verified before deployment.
- **Zero-Knowledge Proofs**: Used for privacy-preserving features.
- **Byzantine Fault Tolerance**: Consensus layer must tolerate malicious nodes.

## Integration & Dependencies
- **External AI Services**: Integrate only with approved AI models for consensus and rewards.
- **Browser Compatibility**: Ensure all code runs efficiently in major browsers.
- **No Gas Fees**: Never introduce transaction fees or require external wallets.

## Key Files & Directories
- `contracts/` — Rust/WASM smart contracts
- `client/` — Browser node implementation
- `ai/` — AI models for consensus and rewards
- `docs/` — Architecture diagrams, specs, and guides


## Example Patterns
- **Global Mining Counter**: UI component (e.g., floating bar/header) with mining count, explanation, and claim button. Example: `client/components/MiningCounter.js`.
- **Action Mining Trigger**: See `client/mining.js` for user interaction hooks.
- **WASM Contract Call**: Reference `contracts/wasm_interface.rs` for invocation patterns.
- **AI Reward Distribution**: Example in `ai/reward_model.py`.


## Contact & Escalation
For unclear patterns or missing documentation, escalate to the project lead or reference the implementation guide in `docs/`.

---
## MVP Testing & Deployment

- Backend: Run `cargo run` in `backend/node`.
- Frontend: Run `npm install && npm run dev` in `client/`.
- AI: Test with `python ai/models/test_reward_model.py`.
- Manual: Interact with the app, mine NUN, and claim via dashboard.

See `docs/MVP_DEPLOY.md` for full instructions.

---
_Last updated: September 2025. Please update this file as the architecture or workflows evolve._

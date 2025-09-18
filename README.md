# UnifiedNUN Project Structure

## Folders
- `client/` — Frontend app (UI, mining counter, dashboard, event handlers)
  - `components/` — Reusable UI components (e.g., MiningCounter)
  - `pages/` — App pages/routes
  - `hooks/` — React/Vue hooks for mining, wallet, etc.
- `backend/` — Blockchain node, mining logic, API
  - `api/` — REST/gRPC endpoints
  - `node/` — Node logic, session/reward tracking
- `contracts/` — Smart contracts (Rust/WASM)
  - `wasm/` — WASM contract code
  - `tests/` — Contract/unit tests
- `ai/` — AI models for consensus and rewards
  - `models/` — ML/AI model code
  - `scripts/` — Training/inference scripts
- `docs/` — Architecture diagrams, specs, guides

## Next Steps
- Scaffold backend and frontend apps
- Implement global mining handler and UI
- Build claim flow and wallet integration
- Add AI reward engine and consensus

See `.github/copilot-instructions.md` for agent guidance.
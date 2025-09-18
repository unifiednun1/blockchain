# UnifiedNUN MVP: Test & Deploy Guide

## 1. Backend
- Run Rust backend node:
  ```
  cd backend/node
  cargo run
  ```
- Endpoints:
  - POST /mine (mining)
  - POST /claim (claim mined NUN)

## 2. Frontend
- Run Next.js frontend:
  ```
  cd client
  npm install
  npm run dev
  ```
- Open http://localhost:3000

## 3. AI Model
- Test reward model:
  ```
  cd ai/models
  python test_reward_model.py
  ```

## 4. Manual Test
- Visit the app, interact, and see the mining counter increase.
- Go to dashboard, enter a wallet, and claim NUN.

## 5. Next Steps
- Integrate real wallet provider
- Harden backend and smart contracts
- Expand AI model and consensus

See README.md and .github/copilot-instructions.md for more details.
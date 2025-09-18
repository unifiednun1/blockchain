# API Routes (Planned)

- `POST /mine` — Trigger mining for a user action (body: session_id, action)
- `POST /claim` — Claim mined NUN (body: session_id, wallet_address)
- `GET /rewards/:session_id` — Get current mined NUN for a session/user

All endpoints will be secured and integrated with session/reward tracking and smart contracts.
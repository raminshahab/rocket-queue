# Rocket Queue (Rust • Rocket • PostgreSQL)

A lightweight multiplayer-ready backend built with **Rust** and **Rocket**, backend by **PostgreSQL** and **Diesel** ORM. Includes database migrations and a clean foundation for real-time or turn-based game logic. **Note this repo has not been updated in a while. Disclaimer: I am also new to Rust in case you see some indiomatic patterns Im not utilizing. As I find time, I will continue to grow this from its current state thanks for your patience** 

## Tech stack

- **Rust** (see `Cargo.toml` for toolchain/edition)
- **Rocket** web framework (async HTTP server & routing)
- **Diesel** ORM + **PostgreSQL** (`diesel.toml`, `migrations/`)

---

## Getting started

### 1) Prerequisites
- Rust toolchain:
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  rustup update
  ```
- PostgreSQL 14+ running locally (or in Docker)

Handy for migrations:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

### 2) Create a database
```bash
createdb rocket_queue_dev
# or:
# psql -c 'CREATE DATABASE game_server_dev;'
```

### 3) Configure environment
Set `DATABASE_URL` (Diesel & Rocket will read it):
```bash
export DATABASE_URL=postgres://localhost/game_server_dev
# $env:DATABASE_URL="postgres://localhost/game_server_dev"
```

If you use Rocket profiles, you can also add a `Rocket.toml`:
```toml
[default]
port = 8000
address = "127.0.0.1"
```

### 4) Run migrations
```bash
diesel setup
diesel migration run
```

> Add a new migration:
> ```bash
> diesel migration generate add_players_table
> ```

### 5) Run the server
```bash
cargo run
# or optimized:
# cargo run --release
```

Server will start on `http://localhost:8000` (or your Rocket profile’s port).

---

## Project layout

```
.
├─ src/                  # Rocket routes, state, domain modules
├─ migrations/           # Diesel migrations (up/down SQL)
├─ Cargo.toml            # Rust deps & metadata
├─ diesel.toml           # Diesel configuration
└─ .gitignore
```

---

## API (starter sketch)

> Routes for fetching and creating players. 

### Health
```
GET /health
200 OK {"status":"ok"}
```

### Players
```
POST /players            # create player
GET  /players/{id}       # fetch player
GET  /players?limit=50   # list players
PATCH /players/{id}      # update fields
DELETE /players/{id}     # soft/hard delete
```

### Sessions / Matchmaking (todo) 
```
POST /sessions
POST /matchmaking/enqueue
DELETE /matchmaking/enqueue
```

---

## Environment variables

| Variable         | Example                                | Notes                          |
|------------------|----------------------------------------|--------------------------------|
| `DATABASE_URL`   | `postgres://localhost/game_server_dev` | Required by Diesel/Rocket      |
| `RUST_LOG`       | `info`                                  | Enable logs via `env_logger`   |
| `ROCKET_PORT`    | `8000`                                  | Overrides `Rocket.toml`        |
| `ROCKET_ADDRESS` | `0.0.0.0`                               | Bind to all interfaces         |

---

## Testing

```bash
cargo test
```

If tests need the DB, point them at a throwaway database:
```bash
createdb game_server_test
export DATABASE_URL=postgres://localhost/game_server_test
diesel migration run
cargo test
```

---
## Running with Docker
```bash
docker build -t game-server .
docker run --rm -p 8000:8000   -e DATABASE_URL=postgres://host.docker.internal/game_server_dev   game-server
```

---

## Common tasks

- New migration: `diesel migration generate <name>`
- Apply migrations: `diesel migration run`
- Revert last: `diesel migration revert`
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`

---

## Roadmap

- [ ] Matchmaking endpoints  
- [ ] Redis ZSets for MMR queues
- [ ] WebSocket or SSE channel for live game state
- [ ] Auth (JWT / session cookies)
- [ ] Rate limiting & request metrics (e.g., `tower`, `prometheus`)
- [ ] E2E tests (pg test container)

---

## Contributing

PRs are welcome. Please run `cargo fmt` and `cargo clippy` before submitting.

---


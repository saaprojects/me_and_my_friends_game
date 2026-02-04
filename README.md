# Me and My Friends (Rust Edition)

Asymmetric ghost‑hunting prototype built with a Bevy client and an Axum server.

## Structure
- `client/` — Bevy 3D prototype
- `server/` — Axum backend (health + websocket stub)
- `shared/` — shared types

## Requirements
- Rust toolchain (install via `rustup`)
- GPU drivers that support Vulkan/DirectX (Bevy renderer)

## Quick Start (Local)
Open two terminals from the repo root.

### 1) Server
```powershell
cargo run -p server
```
Health check: `http://localhost:8000/health`

### 2) Client
```powershell
cargo run -p client
```

## Controls
General
- `Mouse` to look
- `WASD` / arrows to move
- `Shift` to sprint

Menus
- `Esc` toggles the menu (and backs out of detail screens)
- `J` opens the journal (investigator only)

Tools (investigator)
- `1` EMF reader
- `2` Spiritbox
- `E` ask / use spiritbox
- `F` interact

Ghost
- `L` toggle room lights

Debug
- `F3` toggle debug overlay

## Testing
```powershell
cargo test -p shared
cargo test -p server
cargo test -p client
```

Targeted UI tests:
```powershell
cargo test -p client --test ui_flow_tests
```

## Notes
- The client polls the backend health endpoint every 2 seconds.
- The ghost is blocked by outer walls only; the investigator collides with all walls/props.

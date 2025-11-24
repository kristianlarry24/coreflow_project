CoreFlow Unified - Scaffold generated automatically

Directories created:
- backend/        : Rust (Actix-web) backend skeleton
- frontend/       : static web UI (index.html, styles.css, app.js)
- db/             : SQLite schema (schema.sql)

How to build backend:
1. Install Rust (https://rustup.rs) and Cargo.
2. cd backend
3. cargo build --release
4. cargo run  # or run the compiled binary in target/release

How to run locally (dev):
- Start backend (cargo run) and open frontend/index.html via a small static server or configure actix to serve static files.

Notes:
- The backend includes skeleton endpoints: /api/health, /api/sysinfo, /api/clean_temp (dry-run).
- Critical actions such as deleting files are implemented as dry-run only in the scaffold. Before enabling destructive actions, implement backups and confirmation flows.
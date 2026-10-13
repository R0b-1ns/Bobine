# Bobine

Bobine is a macOS app for solo videographers. It keeps a library of your rushes and photos across SSDs, NAS shares, and backup disks — including drives that are unplugged — so you can find a shot by project instead of by folder date.

## Status

Foundations are in progress: SQLite catalog, a local daemon API, and a Tauri shell that can list registered media even when a disk is offline.

Not done yet: automatic volume mapping, clone/backup with hash verification, and the thumbnail browse UI.

## Layout

- `crates/bobine-core` — SQLite catalog
- `crates/bobine-daemon` — API the UI talks to
- `apps/desktop` — Tauri + React app

## Prerequisites

- macOS
- Rust (stable)
- Node.js (for the desktop UI, once it exists)

## Build and test

```bash
cargo test --workspace
```

After the desktop app exists:

```bash
cd apps/desktop
npm test
npm run tauri dev
```

Those `apps/desktop` commands are not available until the Tauri shell is in the tree.

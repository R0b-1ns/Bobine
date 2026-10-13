# Bobine Bible

Bobine is a local library for a solo videographer. It maps rushes and photos by customer project and by still, across disks that may be unplugged, so you can see what you have and where it lives.

## Source of truth

- Spec: `docs/superpowers/specs/2026-10-13-bobine-mvp-design.md`
- Current slice: `docs/superpowers/plans/2026-10-13-bobine-foundations.md`

Do not implement mapping, clone, AI, or NLE plugins unless a plan says so.

## Architecture

- `bobine-core` owns SQLite.
- `bobine-daemon` is the only API the UI may call.
- `bobine-app` (Tauri + React) never opens the database.

A clip is identified by content hash. Locations are that clip on a volume at a path. Photos are clips. Companion files (WAV, XML, LUT, …) are not clips.

## Coding philosophy

- TDD: failing test first, then minimal code, then commit.
- Never write `unsafe` Rust.
- No `unwrap`/`expect` in non-test library paths; return `Result`.
- YAGNI: do not add Qdrant, CoreML, scene labels, or DaVinci code.
- Do not revive the Swift/Xcode app.
- Small focused files; frequent commits.
- Clone (later): destination names are `<stem>.<short-hash>.<ext>`; never overwrite.

## How to work

Read this bible, then the spec, then the active plan. UI depends on `bobine-daemon` only.

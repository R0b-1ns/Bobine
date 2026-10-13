# Bobine Roadmap

Status board for humans and agents. The product spec remains `docs/superpowers/specs/2026-10-13-bobine-mvp-design.md`. This file says what is finished, what to work on now, and what to leave alone.

Agents: read `BIBLE.md`, then this file, then the spec. Implement only the slice under **Doing**. Do not start a **To do** or **Plan to do** item because it looks useful. When a slice starts, move it here to **Doing** and name its plan. When it merges, move it to **Done** with the date and what shipped.

## Done

Shipped. Do not redo.

- **Foundations** (merged 2026-10-13, PR #1). SQLite catalog for volumes, projects, clips, and locations. A clip can be listed when its only disk is disconnected. `bobine-daemon` is the only API the UI may call. Tauri + React shell lists clips as connected or offline. `BIBLE.md` and `README.md` exist.
- **Xcode scaffold removed** (PR #2). The old Swift app is not the product. Do not revive it.

## Doing

Work on this now. One slice at a time.

- **Mapping.** When a card, portable SSD, NAS, or disk already on the Mac appears, Bobine finds new videos and photos by itself. It stores the fat record: time, length, codec, resolution, size, bitrate, fps, camera type, color space, poster, and content hash. Sidecars (WAV, XML, LUT) may be read to fill that record. They are not shown in the app. A disk that goes away stays in the library as offline. Done when plugging a card registers its new clips without typing a path. No plan file yet — write one before coding.

## To do

Still this MVP. Not urgent until **Doing** is empty. Do them in this order.

1. **Clone.** Two jobs, one sheet. A new card asks if you want a copy. You can also pick clips already in the library and copy those onto a connected disk or a folder on the Mac. The same sheet attaches an existing project or creates one. Every file lands as `<stem>.<short-hash>.<ext>` (example: `A001_C001.a1b2c3d4.mov`). Never overwrite. Sidecars copy beside the clip and stay invisible. The copy is not trusted until the hash matches. Same bits on two disks are one clip, two locations. A wiped card that reuses `A001_C001.mov` must not replace the earlier clone.
2. **Browse.** Search owns the main pane. Recent projects and Clone stay in the left sidebar. Results are stills grouped by project. Hover shows length, disk, connected or offline, filename, camera type, and color space. A connected file can be revealed or opened. An offline file still appears and must not pretend it can be opened. A clip can be moved to another project.

After those two, the MVP is usable: plug a card, map it, clone it onto a project, and find the shots later even if the card is gone.

Small cleanup, whenever, not a product slice:

- Accept the Xcode license, or keep building with `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
- Replace the placeholder app icon before showing the app to anyone else.
- Stop committing generated Tauri schemas if they churn every build.
- Drop unused `serde` / `serde_json` on `bobine-app` if they are still unused.

## Plan to do

Future versions. Not this MVP. Do not implement these while Mapping, Clone, or Browse are unfinished.

- Automatic clone rules. Example: an SD card always copies to a working SSD and a backup HDD.
- Auto-named dump projects such as `import-mm-hh-dd-mm-yy`, plus renaming, merging into an existing project, or moving some clips onto a real job after the fact. Moving a clip is already in the MVP. A merge button and generated names are not.
- DaVinci (or other NLE) extension. Non-destructive derush stays in the editor. Bobine renders those timelines so the library holds the chosen b-rolls, not only the raw rushes.
- AI and agents. Scene labels (night, interior, exterior) and search by what is in the frame. The fat clip record exists so this can be added later. No Qdrant, CoreML, or MLX in the MVP.
- A daemon that keeps running when the app is closed.
- Windows or Linux. Tauri stays so a port is possible. This version is macOS only.
- Cloud sync, accounts, or any hosted service. Out of scope.

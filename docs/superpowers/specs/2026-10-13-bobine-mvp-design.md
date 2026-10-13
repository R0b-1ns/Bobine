# Bobine MVP Design

**Date:** 2026-10-13
**Status:** Approved

## Intent

Bobine is a local library for a solo videographer. It exists for the moment you are already in an edit and need a specific shot you know you have.

You do not start from a catalog. You start from a memory: what was in the frame, or which customer job it belonged to. Today that turns into rummaging through folders and trying to reconstruct *when* you shot it. Camera filenames like `A001_C001.mov` do not help.

Bobine is the map of every rush, photo, and b-roll you own — on the Mac, a portable SSD, a NAS, a backup HDD — including disks that are unplugged. It should answer: what you have, which project it belongs to, where it lives, and whether you can grab it right now.

Ingest and clone are how the map stays true. Browse is the moment that matters.

## Who it serves

One person shooting and editing. A “project” is a customer job, not a folder and not a timeline. Example: five shorts for one client are one project. Photos belong in projects the same way videos do.

## Why this shape

- The filesystem is organized by date and dump folder. Memory is organized by picture and job.
- Drives come and go. A catalog that forgets disconnected disks is useless.
- Footage already on the Mac is not a special case. Internal disks are volumes like any other.
- Copies are how work and backup happen. A copy is not trusted until a hash says it matches.
- AI that labels night/interior/exterior, and a DaVinci extension that renders derush timelines, are real ideas. They are not this version. This version stores enough facts that a later agent could use them.

## Goals

1. **Archive** every mapped video and photo with rich technical metadata and a poster still (for a photo, the poster is the image).
2. **Map automatically** when any volume appears — card, portable SSD, NAS, or a disk already on the computer. New media is registered without a hand-built index.
3. **Clone with integrity** in two ways:
   - *Ingest clone:* new **videos and photos** Bobine just detected are offered for copy (not a full volume dump). Destination is a folder on the computer or another connected disk. **Companion files** sitting next to those clips (WAV, XML, LUT, and similar) are copied into the destination folder because they help metadata extraction; they are **not** shown as items in the app.
   - *Library clone:* pick clips already in the library (e.g. b-rolls from a finished job), pick a connected disk (e.g. a just-plugged HDD), copy them there (companions ride along the same way when they exist next to the source file).
   - **Destination names:** every cloned media file is written as `<original-stem>.<short-hash>.<ext>` (example: `A001_C001.a1b2c3d4.mov`). Never write over an existing file. If that name is somehow taken, fail that file rather than overwrite. Sidecars use the **clip’s** short hash in the same pattern (`A001_C001.a1b2c3d4.wav`) so they stay next to the media after a card wipe recreates `A001_C001.mov` with new bits.
4. **Project as the human label.** On the clone/destination sheet, attach an existing project or create a new one. A clip can be moved to another project later.
5. **Browse** without depending on camera names:
   - Opening the app shows a **search panel** as the main feature.
   - **Recent projects** and **Clone** live in a left sidebar so recents can stack without stealing focus.
   - Results are **grouped by project**, shown as **thumbnails**.
   - **Hover** on a still shows length (videos), disk, connected/offline, filename, **camera type**, and **color space**.
6. **Location truth.** A clip on an unplugged drive still appears. A connected clip is easy to get (open / reveal on disk).
7. **Fat records** so future AI agents can locate clips. Store as much as we can extract, even if the grid only shows a still.

## Non-goals (this version)

- DaVinci Resolve (or other NLE) extension; non-destructive derush timeline render into b-rolls.
- AI / scene detection / semantic “what’s in the frame” search (night, interior, exterior, etc.).
- Automated clone *rules* (e.g. “when an SD card appears, always clone to a working SSD and a backup HDD”).
- Auto-created dump projects with generated names like `import-mm-hh-dd-mm-yy`, and merge-into-existing-project as a dedicated automation workflow. (Manual create/select on the popup covers the first version. Clip move covers cleanup.)
- Showing companion files (WAV, XML, LUT, …) inside Bobine. They exist only on disk, in the copy folder.
- Cloning an entire card/volume tree. Only newly detected videos/photos (plus their sidecars on disk).
- Cloud sync, accounts, or any hosted service.
- Shipping Windows/Linux. Tauri is chosen so a later port is possible; this version is macOS.

## Anti-goals

These would count as failure even if the app “works”:

- Only listing files on disks that are currently connected.
- Leading the browse UI with camera filenames.
- Treating a clone as an unrelated second clip with no link to the original.
- Trusting a copy before hash verification finishes.
- Dropping photos out of projects, or ignoring media that already lives on the Mac.
- A Mac-only UI kit that throws away the Tauri portability bet (see Constraints).

## Constraints

- **Tauri** desktop app, because the product should be able to migrate to other platforms later without giving up a fast local core. This is a portability bet, not a request to ship three OSes now.
- **macOS first.** Volume appearance uses DiskArbitration (or equivalent). NAS via Samba counts as a volume. Internal/boot disks count as volumes too.
- **Zero external online dependencies.** Library, hashes, posters, and metadata all live on the machine.
- **Performance stays in Rust.** UI is a client. Catalog, hashing, copying, and volume state live in a Rust core, exposed as a local daemon/API separate from the UI process.
- **SQLite** for metadata, projects, clip records, hashes, and volume identity (disk UUID / stable id, known paths, connected or not).
- There is an older **Swift/Xcode** scaffold in git history (`BobineApp.swift`, etc.), currently deleted in the working tree. Do not revive it. This design replaces that direction.

## Appearance and interaction

### Shell

- Left sidebar: app name, **Clone** always visible, **Recent projects** stacked below.
- Main pane: search first. Recents in the sidebar must not compete with search.

### Browse

- Search filters the library (project name, description, camera type, color space, and other stored text facts — not AI vision).
- Hits appear **under project headings**, as a **thumbnail grid** (videos and photos together).
- Camera names are not the label.
- Hover on a thumbnail: duration when it is a video, volume name, connected vs offline, filename, camera type (iPhone, Sony FX3, Fujifilm X-M5, Lumix S5II, …), color space (Rec.709, Panasonic V-Log, Sony S-Log3, …).
- If a clip has several locations, prefer showing a connected one; still tell the truth if every copy is offline.
- Clicking a recent project in the sidebar is a shortcut into that project’s grid.
- A connected file must be easy to open or reveal in Finder. An offline file must not pretend it can.

### Clone sheet (both jobs)

One accessible Clone action. The sheet covers:

1. What is being copied (new videos/photos just detected, or a selection from the library).
2. Destination: a path on the computer, or a connected volume.
3. Project: pick an existing one or create a new one.

Ingest: when a volume brings **new** videos or photos, Bobine maps them and **asks** whether to clone those new files. Mapping happens even if the answer is no. Project is still attached (existing or new) so new files are not an unlabeled pile. Sidecars copy with a yes, quietly.

Library clone: user selects thumbnails, chooses the connected destination.

Progress must include hash verification of the media files. The copy is not “done” in the library until those hashes match. Sidecars are copied with the media; they need not appear as library rows.

Clone never uses the original filename alone on disk. The short hash in the destination name is a prefix of the clip content hash (length is a builder choice; it must be stable for a given hash). After an SD card is wiped and the camera reuses `A001_C001.mov`, a second clone to the same folder cannot collide with the first file.

## Data: what a clip is

A **clip** is one video or photo, identified by **content hash**. A **location** is that clip on a volume at a path. Cloning adds a location; it does not duplicate the clip.

A **volume** is any disk Bobine has seen: internal Mac disk, card, portable SSD, HDD, or NAS share. It has a stable identity (disk UUID where macOS provides one; a stable share identity for NAS), known mount paths, and connected/disconnected state. Indexing “files already on the computer” is the same mechanism as any other volume, not a separate importer.

A **project** is a user-named customer job. A clip belongs to one project at a time and can be moved.

Companion files are **not clips**. They are extra bytes copied next to media and may be read during extraction.

### Stored on every clip (extract or user)

Technical (extracted when mapped):

- Original filename
- Kind (video or photo)
- Duration / length (video)
- Capture / file time
- Codec
- Resolution
- File size
- Bitrate (video)
- FPS (video)
- **Camera type**
- **Color space**
- Poster (still)
- Content hash
- Any extra technical fields the extractor can fill cheaply — the brief is “maximum data we can have on a clip,” not a closed short list. Sidecars may be used as extraction input.

User:

- Description
- Project (required once mapped)

Location(s):

- Parent volume id
- Path on that volume
- Whether that volume is currently connected

The grid does not dump this list. Camera type and color space **are** shown (hover / clip facts). The rest exists so the app, and later agents, can find and trust the file.

## Flows

### Volume appears (including internal disks already there at launch)

1. Detect volume (connected).
2. Discover video and photo files.
3. Map new files (extract metadata + poster + hash, using sidecars when present; register locations).
4. If new media: prompt clone + project (clone target = those new videos/photos + their sidecars on disk).
5. Known files on a returning volume: mark that volume connected; paths available again.

### Volume disappears

Volume and its locations stay in SQLite. UI shows those clips as offline.

### Move between projects

Change the clip’s project id. No file copy. This is in scope because a name chosen in a hurry on the clone sheet will sometimes be wrong.

## Architecture (decided)

Monorepo / workspace:

- **`bobine-core`** — SQLite, volumes, clips, projects, clone/hash, metadata extraction, posters. No UI.
- **`bobine-daemon`** — background/local API over the core (list volumes, list/search clips, clone, move project, volume events).
- **`bobine-app`** — Tauri app, React + TypeScript UI (this frontend choice is a **design-author call**, not something the partner specified). UI talks to the daemon; it does not own the database.

macOS volume detection sits in core/daemon behind a volume-source interface so a later OS can replace DiskArbitration.

Vector index / Qdrant / CoreML / MLX are out of this version. Keep the door open by not stuffing UI into the core.

### Author calls (not partner decisions)

- **React + TypeScript** inside Tauri, as the default portable UI stack. Partner required Tauri for future platforms; they did not pick React vs Vue vs Svelte.
- **Content hash is clip identity.** Same bits on two disks = one clip, two locations.
- **Hash algorithm:** a fast collision-resistant hash suitable for large video (e.g. BLAKE3 or SHA-256). Builder picks one and uses it everywhere.
- **Poster:** extracted locally at map time (e.g. a frame near the start; the photo itself for stills). Stored in app data, not only next to the media, so offline browse still shows stills.
- **MVP daemon lifetime:** runs with the app. A login-item always-on daemon is later.
- **Project merge** as a single button is not required; moving clips is.
- **Do not revive the Swift app.**
- **Sidecar copy** preserves adjacency in the destination folder by using the clip’s short hash in the sidecar filename; sidecars are not hashed into clip identity.
- **Short hash in clone filenames** is a stable prefix of the content hash; exact character count is a builder choice.
- **Internal volumes** use the same volume pipeline; no separate “index a folder” product.

## What’s left to the builder

- Exact crate/folder layout, Tauri version, IPC (unix socket, localhost HTTP, or Tauri sidecar wrapping the same core).
- How NAS shares are identified when Disk UUID is missing.
- Extractor implementation (AVFoundation, ffmpeg, exif, etc.) as long as it stays local and can later be swapped per OS.
- Which extensions count as video, photo, and sidecar.
- Thumbnail timing, cache layout, and extra hover details.
- Hash streaming while copying vs copy-then-hash, as long as the UI does not mark the clone trusted early. Whether sidecars get their own integrity hash is a builder choice; media hash is required.
- Component styling (native-feeling macOS is desired; pixel-perfect design is not specified).
- Whether “easy get” is Reveal in Finder, open in default player, or both.
- Test strategy, CI, installer/signing.

## Out of scope reminders for planning

Do not plan scene-label models, Qdrant, agent chat, or NLE plugins in the first implementation plan. Do plan schema and APIs that can grow (fat clip rows, multiple locations per clip, volume online flag, photos as clips).

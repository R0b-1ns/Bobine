# Bobine Foundations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A macOS Tauri app whose Rust core can persist volumes, projects, and clips in SQLite and list them even when a volume is disconnected.

**Architecture:** Rust workspace with `bobine-core` (SQLite catalog), `bobine-daemon` (the only API the UI may call), and `bobine-app` (Tauri + React). The UI process does not open the database. This plan does not detect disks, extract metadata, clone, or build the thumbnail browse UI.

**Tech Stack:** Rust 2021, rusqlite (bundled SQLite), serde, thiserror, uuid, Tauri 2, React 18, TypeScript, Vite.

**Spec:** `docs/superpowers/specs/2026-10-13-bobine-mvp-design.md`

## Later plans (not this document)

The approved spec is larger than one testable slice. Implement those as separate plans after this one ships:

1. **Mapping** — DiskArbitration, discover video/photo, extract metadata/poster/hash, sidecars as extraction input.
2. **Clone** — ingest clone of new videos/photos + sidecars, library clone, hash verify, project on the sheet. Destination names are always `<stem>.<short-hash>.<ext>`; never overwrite. A wiped card that reuses `A001_C001.mov` must not replace the earlier clone on disk.
3. **Browse UI** — sidebar recents + Clone, search, grouped thumbnails, hover (camera type, color space, location), move clip between projects.

## Global Constraints

- macOS first; do not ship Windows/Linux in this slice.
- Tauri app; do not revive the deleted Swift/Xcode scaffold in git history.
- UI talks only to `bobine-daemon`; it must not depend on `bobine-core`.
- SQLite for volumes, projects, clips, locations; clip identity will be content hash (nullable until the mapping plan fills it).
- Photos and videos are both clips (`kind`: `video` | `photo`).
- A clone is another location of the same clip, not a second clip (schema must allow multiple locations now).
- Companion files are not clips and are not in this slice’s schema.
- Daemon lifetime: in-process service held by the Tauri app (same crate as a future out-of-process binary).
- Work in git repo `/Users/robis/Project/Bobine/Bobine`. Do not restore `Bobine/*.swift` files.
- Agents must follow `BIBLE.md`. Humans read `README.md`.
- No `unsafe` Rust.

## Review Focus

- A clip whose only volume is disconnected still appears in `list_clips` — Task 2.
- Two different `stable_id` values stay two volumes; inserting the same `stable_id` twice does not create a duplicate volume — Task 2.
- A clip with all optional technical fields `NULL` still lists — Task 2.
- A clip with two locations, only one volume connected, lists once and reports mixed connectivity per location — Task 2.
- The React list marks a clip offline only when every location is disconnected — Task 4.

## File structure

| Path | Responsibility |
|------|----------------|
| `BIBLE.md` | Rules for future agents: product core, architecture, coding philosophy |
| `README.md` | Human intro: what Bobine is, layout, how to build and test |
| `Cargo.toml` | Workspace root: `crates/bobine-core`, `crates/bobine-daemon`, `apps/desktop/src-tauri` |
| `crates/bobine-core/src/schema.sql` | Canonical SQLite schema |
| `crates/bobine-core/src/error.rs` | `Error` / `Result` |
| `crates/bobine-core/src/models.rs` | `Volume`, `VolumeKind`, `ClipKind`, `ClipSummary`, `ClipLocationView`, `Project` |
| `crates/bobine-core/src/db.rs` | Open file, apply schema, `Connection` |
| `crates/bobine-core/src/catalog.rs` | Catalog commands used by the daemon |
| `crates/bobine-core/tests/catalog_list.rs` | List/offline/identity tests |
| `crates/bobine-daemon/src/service.rs` | `BobineService` facade |
| `crates/bobine-daemon/src/lib.rs` | Re-export service + model types for the app |
| `crates/bobine-daemon/tests/service_list.rs` | Offline list through the facade |
| `apps/desktop/src-tauri/` | Tauri host crate `bobine-app`; commands + app state |
| `apps/desktop/src/App.tsx` | Shell: sidebar stub + main list |
| `apps/desktop/src/library/ClipList.tsx` | Renders clip rows and offline state |
| `apps/desktop/src/library/ClipList.test.tsx` | Offline badge rule |

---

### Task 0: Agent bible and human README

**Files:**
- Create: `BIBLE.md`
- Create: `README.md`

**Interfaces:**
- Consumes: nothing
- Produces: two root docs every later task (and later agents) must respect. Do not invent extra process files.

`BIBLE.md` required headings and rules (write them in this order; prose may be short):

1. `# Bobine Bible` — one paragraph: local library for a solo videographer to find rushes/photos by project and still, across disks that may be unplugged.
2. `## Source of truth` — spec is `docs/superpowers/specs/2026-10-13-bobine-mvp-design.md`; current slice is `docs/superpowers/plans/2026-10-13-bobine-foundations.md`. Do not implement mapping, clone, AI, or NLE plugins unless a plan says so.
3. `## Architecture` — `bobine-core` owns SQLite; `bobine-daemon` is the only API the UI may call; `bobine-app` (Tauri + React) never opens the database. Clip = content hash; locations on volumes. Photos are clips. Companion files are not clips.
4. `## Coding philosophy`
   - TDD: failing test first, then minimal code, then commit.
   - Never write `unsafe` Rust.
   - No `unwrap`/`expect` in non-test library paths; return `Result`.
   - YAGNI: do not add Qdrant, CoreML, scene labels, or DaVinci code.
   - Do not revive the Swift/Xcode app.
   - Small focused files; frequent commits.
   - Clone (later): destination names are `<stem>.<short-hash>.<ext>`; never overwrite.
5. `## How to work` — read this bible, then the spec, then the active plan. UI depends on `bobine-daemon` only.

`README.md` required headings:

1. `# Bobine` — what it is, who it is for (human language, not agent rules).
2. `## Status` — foundations in progress; MVP browse/clone/mapping not done.
3. `## Layout` — `crates/bobine-core`, `crates/bobine-daemon`, `apps/desktop`.
4. `## Prerequisites` — macOS, Rust, Node.
5. `## Build and test` — `cargo test --workspace`; after Task 4 also `cd apps/desktop && npm test` and `npm run tauri dev`. Until those exist, say so.

- [ ] **Step 1: Write `BIBLE.md`** with the headings and rules above.

- [ ] **Step 2: Write `README.md`** with the headings above.

- [ ] **Step 3: Verify both files**

Run: `test -f BIBLE.md && test -f README.md && rg -n "^# Bobine Bible|^## Coding philosophy|^# Bobine$|^## Build and test" BIBLE.md README.md`

Expected: both files exist; those headings are present.

- [ ] **Step 4: Commit**

```bash
git add BIBLE.md README.md
git commit -m "docs: add agent bible and human README"
```

---

### Task 1: SQLite catalog opens and migrates

**Files:**
- Create: `Cargo.toml`
- Create: `crates/bobine-core/Cargo.toml`
- Create: `crates/bobine-core/src/lib.rs`
- Create: `crates/bobine-core/src/error.rs`
- Create: `crates/bobine-core/src/db.rs`
- Create: `crates/bobine-core/src/schema.sql`
- Test: `crates/bobine-core/tests/open_db.rs`

**Interfaces:**
- Consumes: nothing
- Produces: `Catalog::open(path: &Path) -> Result<Catalog, Error>` in `crates/bobine-core/src/db.rs` (re-export from `lib.rs`). Second open of the same file succeeds. Schema as below.

Schema (`crates/bobine-core/src/schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS volumes (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  kind TEXT NOT NULL,
  stable_id TEXT NOT NULL UNIQUE,
  last_path TEXT,
  connected INTEGER NOT NULL DEFAULT 0,
  last_seen_at TEXT
);

CREATE TABLE IF NOT EXISTS projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS clips (
  id TEXT PRIMARY KEY,
  content_hash TEXT UNIQUE,
  kind TEXT NOT NULL,
  original_filename TEXT NOT NULL,
  duration_ms INTEGER,
  captured_at TEXT,
  codec TEXT,
  width INTEGER,
  height INTEGER,
  file_size INTEGER,
  bitrate INTEGER,
  fps REAL,
  camera_type TEXT,
  color_space TEXT,
  poster_path TEXT,
  description TEXT,
  project_id TEXT NOT NULL REFERENCES projects(id),
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS clip_locations (
  id TEXT PRIMARY KEY,
  clip_id TEXT NOT NULL REFERENCES clips(id),
  volume_id TEXT NOT NULL REFERENCES volumes(id),
  path TEXT NOT NULL,
  UNIQUE (volume_id, path)
);
```

`VolumeKind` stored as text: `internal` | `external` | `nas` | `card`.
`ClipKind` stored as text: `video` | `photo`.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn open_creates_schema_and_is_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bobine.sqlite");
    bobine_core::Catalog::open(&path).unwrap();
    bobine_core::Catalog::open(&path).unwrap();
    let conn = rusqlite::Connection::open(&path).unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('volumes','projects','clips','clip_locations')",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 4);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p bobine-core --test open_db open_creates_schema_and_is_idempotent -- --nocapture`

Expected: FAIL with unresolved `bobine_core` / `Catalog`

- [ ] **Step 3: Implement `Catalog::open(path: &Path) -> Result<Catalog, Error>`**

Create the workspace + `bobine-core` with deps `rusqlite` (feature `bundled`), `thiserror`, `serde`, `uuid`. `open` creates parent dirs, opens the file, `include_str!`s `schema.sql`, executes it. `Error` is a `thiserror` enum with at least `Sqlite(#[from] rusqlite::Error)` and `Io(#[from] std::io::Error)`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p bobine-core --test open_db open_creates_schema_and_is_idempotent -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/bobine-core
git commit -m "feat: open SQLite catalog with volumes, projects, clips schema"
```

---

### Task 2: List volumes and clips, including disconnected volumes

**Files:**
- Create: `crates/bobine-core/src/models.rs`
- Create: `crates/bobine-core/src/catalog.rs`
- Modify: `crates/bobine-core/src/lib.rs`
- Modify: `crates/bobine-core/src/db.rs` (if `Catalog` needs a `Mutex<Connection>`)
- Test: `crates/bobine-core/tests/catalog_list.rs`

**Interfaces:**
- Consumes: `Catalog::open`
- Produces:

```rust
impl Catalog {
    pub fn insert_project(&self, project: &Project) -> Result<(), Error>;
    pub fn upsert_volume(&self, volume: &Volume) -> Result<(), Error>;
    pub fn insert_clip(&self, clip: &NewClip) -> Result<(), Error>;
    pub fn insert_location(&self, location: &NewLocation) -> Result<(), Error>;
    pub fn list_volumes(&self) -> Result<Vec<Volume>, Error>;
    pub fn list_clips(&self) -> Result<Vec<ClipSummary>, Error>;
}

pub struct Volume {
    pub id: String,
    pub name: String,
    pub kind: VolumeKind,
    pub stable_id: String,
    pub last_path: Option<String>,
    pub connected: bool,
}

pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

pub struct NewClip {
    pub id: String,
    pub content_hash: Option<String>,
    pub kind: ClipKind,
    pub original_filename: String,
    pub project_id: String,
    pub created_at: String,
    // all technical fields optional; omit from constructor via Default
}

pub struct NewLocation {
    pub id: String,
    pub clip_id: String,
    pub volume_id: String,
    pub path: String,
}

pub struct ClipSummary {
    pub id: String,
    pub kind: ClipKind,
    pub original_filename: String,
    pub project_id: String,
    pub project_name: String,
    pub duration_ms: Option<i64>,
    pub camera_type: Option<String>,
    pub color_space: Option<String>,
    pub locations: Vec<ClipLocationView>,
}

pub struct ClipLocationView {
    pub volume_id: String,
    pub volume_name: String,
    pub path: String,
    pub connected: bool,
}
```

All of `Volume`, `ClipSummary`, `ClipLocationView`, and `Project` derive `Serialize, Deserialize` with `#[serde(rename_all = "camelCase")]`. `VolumeKind` and `ClipKind` serialize as `"internal" | "external" | "nas" | "card"` and `"video" | "photo"`.

`upsert_volume` matches on `stable_id`: update name/path/connected if it exists, otherwise insert. `list_clips` returns every clip; each location carries that volume’s current `connected` flag. Never hide a clip because `connected` is false. Optional technical fields on `NewClip` are `Option` with `Default`.

- [ ] **Step 1: Write the failing tests**

Fixture names from the spec:

- Project `"Martin — 5 shorts"`
- Volume `"Backup HDD"`, `stable_id = "HDD-UUID-1"`, `connected = false`, `last_path = "/Volumes/Backup"`
- Volume `"SSD 2"`, `stable_id = "SSD-UUID-2"`, `connected = true`, `last_path = "/Volumes/SSD2"`
- Clip `A001_C001.mov` (video, no technical fields) on Backup HDD at `A001_C001.mov`
- Clip `street_dusk.mov` (video) on SSD 2 at `broll/street_dusk.mov`
- Photo `cafe_window.jpg` on both volumes (two locations)

```rust
#[test]
fn lists_clip_when_its_only_volume_is_disconnected() { /* A001_C001.mov present; its location.connected == false */ }

#[test]
fn upsert_volume_same_stable_id_does_not_duplicate() {
    // upsert Backup HDD twice, second time connected=true, name still "Backup HDD"
    // list_volumes len == 1 and connected == true
}

#[test]
fn clip_with_null_technical_fields_lists() { /* A001_C001.mov: duration_ms, camera_type, color_space are None */ }

#[test]
fn clip_with_two_locations_lists_once_with_mixed_connectivity() {
    // cafe_window.jpg appears once
    // locations: one connected true (SSD 2), one false (Backup HDD)
}

#[test]
fn photo_kind_lists() { /* cafe_window.jpg kind == Photo */ }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p bobine-core --test catalog_list -- --nocapture`

Expected: FAIL (methods/types missing)

- [ ] **Step 3: Implement the `Catalog` methods in `crates/bobine-core/src/catalog.rs`**

Use a `Mutex<rusqlite::Connection>` inside `Catalog` so `&self` works. `list_clips` joins `clips`, `projects`, `clip_locations`, `volumes` and groups locations in Rust by `clip_id` (order: project name, filename). `upsert_volume` is `INSERT ... ON CONFLICT(stable_id) DO UPDATE`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p bobine-core --test catalog_list -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/bobine-core
git commit -m "feat: list volumes and clips including disconnected disks"
```

---

### Task 3: Daemon facade the UI will call

**Files:**
- Create: `crates/bobine-daemon/Cargo.toml`
- Create: `crates/bobine-daemon/src/lib.rs`
- Create: `crates/bobine-daemon/src/service.rs`
- Test: `crates/bobine-daemon/tests/service_list.rs`
- Modify: `Cargo.toml` (workspace members)

**Interfaces:**
- Consumes: `Catalog` methods from Task 2
- Produces:

```rust
pub struct BobineService { /* owns Catalog */ }

impl BobineService {
    pub fn open(db_path: &Path) -> Result<Self, bobine_core::Error>;
    pub fn list_volumes(&self) -> Result<Vec<Volume>, bobine_core::Error>;
    pub fn list_clips(&self) -> Result<Vec<ClipSummary>, bobine_core::Error>;
}
```

Re-export `Volume`, `ClipSummary`, `ClipLocationView`, `ClipKind`, `VolumeKind` from `bobine-daemon` so `bobine-app` never imports `bobine-core`. `Cargo.toml` of the daemon depends on `bobine-core`. Do not add HTTP/unix socket in this slice.

- [ ] **Step 1: Write the failing test**

Open a temp DB with `bobine_core::Catalog`, seed the Martin fixture using Task 2 insert methods, drop that catalog, then `BobineService::open` on the same path. Daemon unit tests may use `bobine_core`; `bobine-app` must not. Assert `list_clips` includes `A001_C001.mov` with `locations[0].connected == false`.

```rust
#[test]
fn service_lists_offline_clip() { /* ... */ }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p bobine-daemon --test service_list service_lists_offline_clip -- --nocapture`

Expected: FAIL with unresolved `bobine_daemon`

- [ ] **Step 3: Implement `BobineService` in `crates/bobine-daemon/src/service.rs`**

Thin wrapper: `open` calls `Catalog::open`; list methods delegate.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p bobine-daemon --test service_list service_lists_offline_clip -- --nocapture`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/bobine-daemon
git commit -m "feat: daemon service lists catalog without UI owning SQLite"
```

---

### Task 4: Tauri + React shell lists library and offline state

**Files:**
- Create: `apps/desktop/package.json`
- Create: `apps/desktop/vite.config.ts`
- Create: `apps/desktop/tsconfig.json`
- Create: `apps/desktop/index.html`
- Create: `apps/desktop/src/main.tsx`
- Create: `apps/desktop/src/App.tsx`
- Create: `apps/desktop/src/library/ClipList.tsx`
- Create: `apps/desktop/src/library/types.ts`
- Test: `apps/desktop/src/library/ClipList.test.tsx`
- Create: `apps/desktop/src-tauri/Cargo.toml` (package name `bobine-app`)
- Create: `apps/desktop/src-tauri/src/main.rs`
- Create: `apps/desktop/src-tauri/src/lib.rs`
- Create: `apps/desktop/src-tauri/tauri.conf.json`
- Create: `apps/desktop/src-tauri/build.rs`
- Modify: `Cargo.toml` workspace members to include `apps/desktop/src-tauri`
- Create: `.gitignore` entries for `target/`, `node_modules/`, `apps/desktop/dist/`, `.superpowers/` if missing

**Interfaces:**
- Consumes: `BobineService::open`, `list_volumes`, `list_clips`
- Produces: Tauri commands `list_volumes` and `list_clips` returning the daemon types as camelCase JSON (`originalFilename`, `projectName`, `durationMs`, `cameraType`, `colorSpace`, `volumeName`).

Rust commands in `apps/desktop/src-tauri/src/lib.rs`:

```rust
#[tauri::command]
fn list_volumes(service: tauri::State<BobineService>) -> Result<Vec<Volume>, String>;

#[tauri::command]
fn list_clips(service: tauri::State<BobineService>) -> Result<Vec<ClipSummary>, String>;
```

On setup, `BobineService::open` uses `app.path().app_data_dir()?.join("bobine.sqlite")`. `bobine-app` depends on `bobine-daemon` only.

UI for this slice (not the final browse spec):

- Left sidebar: text `Bobine`, a non-functional `Clone` label, a `Recent` heading with no projects yet.
- Main pane: a search input that does not filter yet; below it a list from `list_clips`.
- Each row: `originalFilename`, `projectName`, and either `connected` (if any location `connected`) or `offline` (if every location is disconnected).

- [ ] **Step 1: Write the failing test**

`apps/desktop/src/library/ClipList.test.tsx` with React Testing Library + vitest:

```tsx
it("shows offline only when every location is disconnected", () => {
  render(
    <ClipList
      clips={[
        {
          id: "c1",
          kind: "video",
          originalFilename: "A001_C001.mov",
          projectId: "p1",
          projectName: "Martin — 5 shorts",
          durationMs: null,
          cameraType: null,
          colorSpace: null,
          locations: [
            { volumeId: "v1", volumeName: "Backup HDD", path: "A001_C001.mov", connected: false },
          ],
        },
        {
          id: "c2",
          kind: "video",
          originalFilename: "street_dusk.mov",
          projectId: "p1",
          projectName: "Martin — 5 shorts",
          durationMs: null,
          cameraType: null,
          colorSpace: null,
          locations: [
            { volumeId: "v2", volumeName: "SSD 2", path: "broll/street_dusk.mov", connected: true },
          ],
        },
      ]}
    />,
  );
  expect(screen.getByText("A001_C001.mov").closest("li")).toHaveTextContent(/offline/i);
  expect(screen.getByText("street_dusk.mov").closest("li")).toHaveTextContent(/connected/i);
  expect(screen.getByText("street_dusk.mov").closest("li")).not.toHaveTextContent(/offline/i);
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd apps/desktop && npx vitest run src/library/ClipList.test.tsx`

Expected: FAIL (file or component missing). Add vitest, jsdom, `@testing-library/react` as devDependencies as part of this step’s scaffolding if the command cannot run yet — still fail on missing `ClipList`.

- [ ] **Step 3: Implement `ClipList` and the Tauri app**

`fn clip_status(locations: &[ClipLocationView]) -> &'static str` logic lives in the React component: `offline` iff `locations.length > 0 && locations.every(l => !l.connected)`. Scaffold Tauri 2 + Vite React TS so `npm run tauri build` is not required to pass this task; `cargo test --workspace` and `npx vitest run` are. Wire commands; `App.tsx` calls `invoke("list_clips")` and `invoke("list_volumes")` on mount and passes clips to `ClipList`. Empty library is valid (no seed in production).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --workspace`

Run: `cd apps/desktop && npx vitest run src/library/ClipList.test.tsx`

Expected: PASS both

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml .gitignore apps/desktop crates
git commit -m "feat: Tauri shell lists clips with connected vs offline"
```

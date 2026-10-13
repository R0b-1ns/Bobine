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

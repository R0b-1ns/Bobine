use rusqlite::params;

use crate::db::Catalog;
use crate::error::Result;
use crate::models::{ClipLocationView, ClipSummary, NewClip, NewLocation, Project, Volume};

impl Catalog {
    pub fn insert_project(&self, project: &Project) -> Result<()> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO projects (id, name, created_at) VALUES (?1, ?2, ?3)",
            params![project.id, project.name, project.created_at],
        )?;
        Ok(())
    }

    pub fn upsert_volume(&self, volume: &Volume) -> Result<()> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO volumes (id, name, kind, stable_id, last_path, connected)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(stable_id) DO UPDATE SET
               name = excluded.name,
               last_path = excluded.last_path,
               connected = excluded.connected",
            params![
                volume.id,
                volume.name,
                volume.kind,
                volume.stable_id,
                volume.last_path,
                i64::from(volume.connected),
            ],
        )?;
        Ok(())
    }

    pub fn insert_clip(&self, clip: &NewClip) -> Result<()> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO clips (
                id, content_hash, kind, original_filename, duration_ms, captured_at,
                codec, width, height, file_size, bitrate, fps, camera_type, color_space,
                poster_path, description, project_id, created_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18
             )",
            params![
                clip.id,
                clip.content_hash,
                clip.kind,
                clip.original_filename,
                clip.duration_ms,
                clip.captured_at,
                clip.codec,
                clip.width,
                clip.height,
                clip.file_size,
                clip.bitrate,
                clip.fps,
                clip.camera_type,
                clip.color_space,
                clip.poster_path,
                clip.description,
                clip.project_id,
                clip.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn insert_location(&self, location: &NewLocation) -> Result<()> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO clip_locations (id, clip_id, volume_id, path) VALUES (?1, ?2, ?3, ?4)",
            params![location.id, location.clip_id, location.volume_id, location.path],
        )?;
        Ok(())
    }

    pub fn list_volumes(&self) -> Result<Vec<Volume>> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, kind, stable_id, last_path, connected
             FROM volumes
             ORDER BY name",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Volume {
                id: row.get(0)?,
                name: row.get(1)?,
                kind: row.get(2)?,
                stable_id: row.get(3)?,
                last_path: row.get(4)?,
                connected: row.get::<_, i64>(5)? != 0,
            })
        })?;
        let mut volumes = Vec::new();
        for row in rows {
            volumes.push(row?);
        }
        Ok(volumes)
    }

    pub fn list_clips(&self) -> Result<Vec<ClipSummary>> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "SELECT
                c.id, c.kind, c.original_filename, c.project_id, p.name,
                c.duration_ms, c.camera_type, c.color_space,
                l.volume_id, v.name, l.path, v.connected
             FROM clips c
             JOIN projects p ON p.id = c.project_id
             LEFT JOIN clip_locations l ON l.clip_id = c.id
             LEFT JOIN volumes v ON v.id = l.volume_id
             ORDER BY p.name, c.original_filename, l.path",
        )?;
        let rows = stmt.query_map([], |row| {
            let volume_id: Option<String> = row.get(8)?;
            let location = match volume_id {
                Some(volume_id) => Some(ClipLocationView {
                    volume_id,
                    volume_name: row.get(9)?,
                    path: row.get(10)?,
                    connected: row.get::<_, Option<i64>>(11)?.unwrap_or(0) != 0,
                }),
                None => None,
            };
            Ok((
                ClipSummary {
                    id: row.get(0)?,
                    kind: row.get(1)?,
                    original_filename: row.get(2)?,
                    project_id: row.get(3)?,
                    project_name: row.get(4)?,
                    duration_ms: row.get(5)?,
                    camera_type: row.get(6)?,
                    color_space: row.get(7)?,
                    locations: Vec::new(),
                },
                location,
            ))
        })?;

        let mut clips: Vec<ClipSummary> = Vec::new();
        for row in rows {
            let (summary, location) = row?;
            if let Some(last) = clips.last_mut() {
                if last.id == summary.id {
                    if let Some(location) = location {
                        last.locations.push(location);
                    }
                    continue;
                }
            }
            let mut summary = summary;
            if let Some(location) = location {
                summary.locations.push(location);
            }
            clips.push(summary);
        }
        Ok(clips)
    }
}

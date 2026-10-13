use bobine_core::{Catalog, ClipKind, NewClip, NewLocation, Project, Volume, VolumeKind};

fn open() -> (tempfile::TempDir, Catalog) {
    let dir = tempfile::tempdir().unwrap();
    let catalog = Catalog::open(&dir.path().join("bobine.sqlite")).unwrap();
    (dir, catalog)
}

fn martin() -> Project {
    Project {
        id: "p-martin".into(),
        name: "Martin — 5 shorts".into(),
        created_at: "2026-10-13T00:00:00Z".into(),
    }
}

fn backup_hdd(connected: bool) -> Volume {
    Volume {
        id: "v-hdd".into(),
        name: "Backup HDD".into(),
        kind: VolumeKind::External,
        stable_id: "HDD-UUID-1".into(),
        last_path: Some("/Volumes/Backup".into()),
        connected,
    }
}

fn ssd2() -> Volume {
    Volume {
        id: "v-ssd".into(),
        name: "SSD 2".into(),
        kind: VolumeKind::External,
        stable_id: "SSD-UUID-2".into(),
        last_path: Some("/Volumes/SSD2".into()),
        connected: true,
    }
}

fn video(id: &str, filename: &str) -> NewClip {
    NewClip {
        id: id.into(),
        kind: ClipKind::Video,
        original_filename: filename.into(),
        project_id: "p-martin".into(),
        created_at: "2026-10-13T00:00:00Z".into(),
        ..NewClip::default()
    }
}

#[test]
fn lists_clip_when_its_only_volume_is_disconnected() {
    let (_dir, catalog) = open();
    catalog.insert_project(&martin()).unwrap();
    catalog.upsert_volume(&backup_hdd(false)).unwrap();
    catalog.insert_clip(&video("c-a001", "A001_C001.mov")).unwrap();
    catalog
        .insert_location(&NewLocation {
            id: "loc-a001".into(),
            clip_id: "c-a001".into(),
            volume_id: "v-hdd".into(),
            path: "A001_C001.mov".into(),
        })
        .unwrap();

    let clips = catalog.list_clips().unwrap();
    let clip = clips.iter().find(|c| c.original_filename == "A001_C001.mov").unwrap();
    assert_eq!(clip.locations.len(), 1);
    assert!(!clip.locations[0].connected);
    assert_eq!(clip.locations[0].volume_name, "Backup HDD");
}

#[test]
fn upsert_volume_same_stable_id_does_not_duplicate() {
    let (_dir, catalog) = open();
    catalog.upsert_volume(&backup_hdd(false)).unwrap();
    let mut again = backup_hdd(true);
    again.id = "v-hdd-second".into();
    catalog.upsert_volume(&again).unwrap();

    let volumes = catalog.list_volumes().unwrap();
    assert_eq!(volumes.len(), 1);
    assert_eq!(volumes[0].name, "Backup HDD");
    assert_eq!(volumes[0].stable_id, "HDD-UUID-1");
    assert!(volumes[0].connected);
}

#[test]
fn clip_with_null_technical_fields_lists() {
    let (_dir, catalog) = open();
    catalog.insert_project(&martin()).unwrap();
    catalog.upsert_volume(&backup_hdd(false)).unwrap();
    catalog.insert_clip(&video("c-a001", "A001_C001.mov")).unwrap();
    catalog
        .insert_location(&NewLocation {
            id: "loc-a001".into(),
            clip_id: "c-a001".into(),
            volume_id: "v-hdd".into(),
            path: "A001_C001.mov".into(),
        })
        .unwrap();

    let clip = catalog
        .list_clips()
        .unwrap()
        .into_iter()
        .find(|c| c.original_filename == "A001_C001.mov")
        .unwrap();
    assert_eq!(clip.duration_ms, None);
    assert_eq!(clip.camera_type, None);
    assert_eq!(clip.color_space, None);
}

#[test]
fn clip_with_two_locations_lists_once_with_mixed_connectivity() {
    let (_dir, catalog) = open();
    catalog.insert_project(&martin()).unwrap();
    catalog.upsert_volume(&backup_hdd(false)).unwrap();
    catalog.upsert_volume(&ssd2()).unwrap();
    catalog
        .insert_clip(&NewClip {
            id: "c-cafe".into(),
            kind: ClipKind::Photo,
            original_filename: "cafe_window.jpg".into(),
            project_id: "p-martin".into(),
            created_at: "2026-10-13T00:00:00Z".into(),
            ..NewClip::default()
        })
        .unwrap();
    catalog
        .insert_location(&NewLocation {
            id: "loc-cafe-hdd".into(),
            clip_id: "c-cafe".into(),
            volume_id: "v-hdd".into(),
            path: "stills/cafe_window.jpg".into(),
        })
        .unwrap();
    catalog
        .insert_location(&NewLocation {
            id: "loc-cafe-ssd".into(),
            clip_id: "c-cafe".into(),
            volume_id: "v-ssd".into(),
            path: "stills/cafe_window.jpg".into(),
        })
        .unwrap();

    let clips: Vec<_> = catalog
        .list_clips()
        .unwrap()
        .into_iter()
        .filter(|c| c.original_filename == "cafe_window.jpg")
        .collect();
    assert_eq!(clips.len(), 1);
    let connected: Vec<bool> = clips[0].locations.iter().map(|l| l.connected).collect();
    assert!(connected.contains(&true));
    assert!(connected.contains(&false));
    assert_eq!(clips[0].locations.len(), 2);
}

#[test]
fn photo_kind_lists() {
    let (_dir, catalog) = open();
    catalog.insert_project(&martin()).unwrap();
    catalog.upsert_volume(&ssd2()).unwrap();
    catalog
        .insert_clip(&NewClip {
            id: "c-cafe".into(),
            kind: ClipKind::Photo,
            original_filename: "cafe_window.jpg".into(),
            project_id: "p-martin".into(),
            created_at: "2026-10-13T00:00:00Z".into(),
            ..NewClip::default()
        })
        .unwrap();
    catalog
        .insert_location(&NewLocation {
            id: "loc-cafe-ssd".into(),
            clip_id: "c-cafe".into(),
            volume_id: "v-ssd".into(),
            path: "stills/cafe_window.jpg".into(),
        })
        .unwrap();

    let clip = catalog
        .list_clips()
        .unwrap()
        .into_iter()
        .find(|c| c.original_filename == "cafe_window.jpg")
        .unwrap();
    assert_eq!(clip.kind, ClipKind::Photo);
}

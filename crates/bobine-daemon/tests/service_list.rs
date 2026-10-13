use bobine_core::{Catalog, ClipKind, NewClip, NewLocation, Project, Volume, VolumeKind};

#[test]
fn service_lists_offline_clip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bobine.sqlite");
    {
        let catalog = Catalog::open(&path).unwrap();
        catalog
            .insert_project(&Project {
                id: "p-martin".into(),
                name: "Martin — 5 shorts".into(),
                created_at: "2026-10-13T00:00:00Z".into(),
            })
            .unwrap();
        catalog
            .upsert_volume(&Volume {
                id: "v-hdd".into(),
                name: "Backup HDD".into(),
                kind: VolumeKind::External,
                stable_id: "HDD-UUID-1".into(),
                last_path: Some("/Volumes/Backup".into()),
                connected: false,
            })
            .unwrap();
        catalog
            .insert_clip(&NewClip {
                id: "c-a001".into(),
                kind: ClipKind::Video,
                original_filename: "A001_C001.mov".into(),
                project_id: "p-martin".into(),
                created_at: "2026-10-13T00:00:00Z".into(),
                ..NewClip::default()
            })
            .unwrap();
        catalog
            .insert_location(&NewLocation {
                id: "loc-a001".into(),
                clip_id: "c-a001".into(),
                volume_id: "v-hdd".into(),
                path: "A001_C001.mov".into(),
            })
            .unwrap();
    }

    let service = bobine_daemon::BobineService::open(&path).unwrap();
    let clips = service.list_clips().unwrap();
    let clip = clips
        .iter()
        .find(|clip| clip.original_filename == "A001_C001.mov")
        .unwrap();
    assert_eq!(clip.locations.len(), 1);
    assert!(!clip.locations[0].connected);
}

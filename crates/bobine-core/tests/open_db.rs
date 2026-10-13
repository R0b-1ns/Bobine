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

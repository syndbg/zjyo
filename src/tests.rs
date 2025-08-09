use crate::{DirEntry, ZDatabase};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn create_test_db() -> ZDatabase {
    let temp_file = format!(
        "/tmp/test_z_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let mut db = ZDatabase {
        entries: HashMap::new(),
        data_file: PathBuf::from(&temp_file),
    };

    // Add some test entries
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    db.entries.insert(
        "/home/user/projects".to_string(),
        DirEntry::new("/home/user/projects".to_string(), 5.0, now - 1000),
    );
    db.entries.insert(
        "/home/user/documents".to_string(),
        DirEntry::new("/home/user/documents".to_string(), 3.0, now - 2000),
    );
    db.entries.insert(
        "/home/user/downloads".to_string(),
        DirEntry::new("/home/user/downloads".to_string(), 7.0, now - 500),
    );

    db
}

#[test]
fn test_dir_entry_frecency() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let entry = DirEntry::new("/test/path".to_string(), 10.0, now - 1000);
    let frecency = entry.frecency();
    assert!(frecency > 0);

    // More recent entry should have higher frecency than older one with same rank
    let recent_entry = DirEntry::new("/test/path2".to_string(), 10.0, now - 100);
    let recent_frecency = recent_entry.frecency();
    assert!(recent_frecency > frecency);
}

#[test]
fn test_database_add_new_directory() {
    let mut db = create_test_db();
    let initial_count = db.entries.len();

    db.add("/new/directory");

    assert_eq!(db.entries.len(), initial_count + 1);
    assert!(db.entries.contains_key("/new/directory"));
    assert_eq!(db.entries["/new/directory"].rank, 1.0);
}

#[test]
fn test_database_add_existing_directory() {
    let mut db = create_test_db();
    let initial_rank = db.entries["/home/user/projects"].rank;

    db.add("/home/user/projects");

    assert_eq!(db.entries["/home/user/projects"].rank, initial_rank + 1.0);
}

#[test]
fn test_database_remove() {
    let mut db = create_test_db();
    assert!(db.entries.contains_key("/home/user/projects"));

    db.remove("/home/user/projects");

    assert!(!db.entries.contains_key("/home/user/projects"));
}

#[test]
fn test_find_matches_basic() {
    let db = create_test_db();

    let matches = db.find_matches("proj", None);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/projects");
}

#[test]
fn test_find_matches_multiple() {
    let db = create_test_db();

    let matches = db.find_matches("user", None);

    assert_eq!(matches.len(), 3);
    // Should be sorted by frecency (downloads is most recent, so highest frecency)
    assert_eq!(matches[0].path, "/home/user/downloads");
}

#[test]
fn test_find_matches_case_insensitive() {
    let db = create_test_db();

    let matches = db.find_matches("PROJ", None);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/projects");
}

#[test]
fn test_find_by_rank() {
    let db = create_test_db();

    let matches = db.find_by_rank("user");

    assert_eq!(matches.len(), 3);
    // Should be sorted by rank (downloads has highest rank: 7.0)
    assert_eq!(matches[0].path, "/home/user/downloads");
    assert_eq!(matches[1].path, "/home/user/projects"); // rank 5.0
    assert_eq!(matches[2].path, "/home/user/documents"); // rank 3.0
}

#[test]
fn test_find_by_time() {
    let db = create_test_db();

    let matches = db.find_by_time("user");

    assert_eq!(matches.len(), 3);
    // Should be sorted by time (downloads is most recent)
    assert_eq!(matches[0].path, "/home/user/downloads");
}

#[test]
fn test_aging_when_rank_exceeds_limit() {
    let mut db = create_test_db();

    // Add entries to exceed 9000 total rank
    for i in 0..3000 {
        db.entries.insert(
            format!("/test/path/{}", i),
            DirEntry::new(
                format!("/test/path/{}", i),
                3.5,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        );
    }

    // Verify total rank exceeds 9000
    let total_rank: f64 = db.entries.values().map(|e| e.rank).sum();
    assert!(total_rank > 9000.0);

    let initial_count = db.entries.len();
    db.add("/new/test/path");

    // After aging, some entries should be removed (those with rank < 1.0 after multiplying by 0.99)
    assert!(db.entries.len() <= initial_count);

    // All remaining entries should have rank >= 1.0
    for entry in db.entries.values() {
        assert!(entry.rank >= 1.0);
    }
}

#[test]
fn test_save_and_load() {
    let temp_file = format!(
        "/tmp/test_z_save_load_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let mut db = ZDatabase {
        entries: HashMap::new(),
        data_file: PathBuf::from(&temp_file),
    };

    // Add some test data
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    db.entries.insert(
        "/test/path1".to_string(),
        DirEntry::new("/test/path1".to_string(), 5.0, now),
    );
    db.entries.insert(
        "/test/path2".to_string(),
        DirEntry::new("/test/path2".to_string(), 3.0, now - 1000),
    );

    // Save to file
    db.save();

    // Create new database and load
    let mut db2 = ZDatabase {
        entries: HashMap::new(),
        data_file: PathBuf::from(&temp_file),
    };
    db2.load();

    // Should have same entries
    assert_eq!(db2.entries.len(), 2);
    assert!(db2.entries.contains_key("/test/path1"));
    assert!(db2.entries.contains_key("/test/path2"));
    assert_eq!(db2.entries["/test/path1"].rank, 5.0);
    assert_eq!(db2.entries["/test/path2"].rank, 3.0);

    // Cleanup
    fs::remove_file(&temp_file).ok();
}

#[test]
fn test_current_directory_restriction() {
    let mut db = create_test_db();
    db.entries.insert(
        "/home/user/projects/rust".to_string(),
        DirEntry::new(
            "/home/user/projects/rust".to_string(),
            2.0,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ),
    );

    let matches = db.find_matches("rust", Some("/home/user/projects"));

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/projects/rust");

    // Should not match if current dir doesn't match
    let matches = db.find_matches("rust", Some("/home/other"));
    assert_eq!(matches.len(), 0);
}

#[test]
fn test_empty_pattern() {
    let db = create_test_db();

    let matches = db.find_matches("", None);

    // Empty pattern should match all entries
    assert_eq!(matches.len(), 3);
}

#[test]
fn test_multi_word_pattern_matching() {
    let mut db = create_test_db();

    // Add some entries with more complex paths
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    db.entries.insert(
        "/home/user/projects/rust-web-app".to_string(),
        DirEntry::new("/home/user/projects/rust-web-app".to_string(), 5.0, now),
    );
    db.entries.insert(
        "/home/user/docs/api-reference".to_string(),
        DirEntry::new("/home/user/docs/api-reference".to_string(), 3.0, now),
    );

    // Test multi-word matching
    let matches = db.find_matches("rust web", None);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/projects/rust-web-app");

    let matches = db.find_matches("api reference", None);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/docs/api-reference");

    // Test that both words must be present
    let matches = db.find_matches("rust database", None);
    assert_eq!(matches.len(), 0);

    // Test case insensitive multi-word matching
    let matches = db.find_matches("RUST WEB", None);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].path, "/home/user/projects/rust-web-app");
}

#[test]
fn test_no_matches() {
    let db = create_test_db();

    let matches = db.find_matches("nonexistent", None);

    assert_eq!(matches.len(), 0);
}

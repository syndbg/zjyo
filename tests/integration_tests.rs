use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

fn get_binary_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("target");

    // Check if we're in release mode first, then fall back to debug
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    path.push(profile);
    path.push("zjyo");

    // If the binary doesn't exist in the expected location, try the other profile
    if !path.exists() {
        let mut fallback_path = env::current_dir().unwrap();
        fallback_path.push("target");
        let fallback_profile = if profile == "debug" {
            "release"
        } else {
            "debug"
        };
        fallback_path.push(fallback_profile);
        fallback_path.push("zjyo");

        if fallback_path.exists() {
            return fallback_path;
        }
    }

    path
}

fn create_temp_data_file() -> String {
    format!(
        "/tmp/test_zjyo_integration_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

#[test]
fn test_help_output() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let help_text = String::from_utf8(output.stdout).unwrap();
    assert!(help_text.contains("jump around faster"));
    assert!(help_text.contains("Directory pattern to match"));
    assert!(help_text.contains("-l"));
    assert!(help_text.contains("-r"));
    assert!(help_text.contains("-t"));
    assert!(help_text.contains("--doctor"));
}

#[test]
fn test_add_and_list() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let test_dir = temp_dir.path().join("test_dir");
    fs::create_dir(&test_dir).unwrap();
    let test_dir = test_dir.to_string_lossy().to_string();

    // Create test database with known entry
    let test_db_content = format!("{}|1.0|1640995200\n", test_dir);
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // List directories
    let output = Command::new(get_binary_path())
        .arg("-l")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let list_output = String::from_utf8(output.stdout).unwrap();
    assert!(list_output.contains(&test_dir));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_doctor_removes_missing_directories() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let existing = temp_dir.path().join("work");
    let missing = temp_dir.path().join("example-");
    fs::create_dir(&existing).unwrap();
    let existing = existing.to_string_lossy().to_string();
    let missing = missing.to_string_lossy().to_string();

    fs::write(
        &temp_data,
        format!(
            "{}|9999.0|1640995200\n{}|1.0|1640995200\n",
            missing, existing
        ),
    )
    .expect("Failed to write test data");

    let output = Command::new(get_binary_path())
        .arg("--doctor")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout).unwrap().trim().is_empty());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Removed 1 stale entries"));

    let data = fs::read_to_string(&temp_data).unwrap();
    assert!(!data.contains(&missing));
    assert!(data.contains(&existing));

    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_pattern_matching() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let projects = temp_dir.path().join("projects");
    let documents = temp_dir.path().join("documents");
    fs::create_dir(&projects).unwrap();
    fs::create_dir(&documents).unwrap();
    let projects = projects.to_string_lossy().to_string();
    let documents = documents.to_string_lossy().to_string();

    // Manually create a test database file
    let test_db_content = format!(
        "{}|5.0|1640995200\n{}|3.0|1640995100\n",
        projects, documents
    );
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Test pattern matching
    let output = Command::new(get_binary_path())
        .arg("-e")
        .arg("proj")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let match_output = String::from_utf8(output.stdout).unwrap();
    assert!(match_output.contains(&projects));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_rank_sorting() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path().join("path");
    let low = base.join("low");
    let high = base.join("high");
    let medium = base.join("medium");
    fs::create_dir_all(&low).unwrap();
    fs::create_dir_all(&high).unwrap();
    fs::create_dir_all(&medium).unwrap();
    let low = low.to_string_lossy().to_string();
    let high = high.to_string_lossy().to_string();
    let medium = medium.to_string_lossy().to_string();

    // Create database with different ranks
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_db_content = format!(
        "{}|2.0|{}\n{}|10.0|{}\n{}|5.0|{}\n",
        low, now, high, now, medium, now
    );
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Test rank-based sorting
    let output = Command::new(get_binary_path())
        .arg("-r")
        .arg("-l")
        .arg("path")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let list_output = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = list_output.lines().collect();

    // Should be sorted by rank (highest first)
    assert!(lines[0].contains(&high));
    assert!(lines[1].contains(&medium));
    assert!(lines[2].contains(&low));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_time_sorting() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path().join("path");
    let old = base.join("old");
    let new = base.join("new");
    let middle = base.join("middle");
    fs::create_dir_all(&old).unwrap();
    fs::create_dir_all(&new).unwrap();
    fs::create_dir_all(&middle).unwrap();
    let old = old.to_string_lossy().to_string();
    let new = new.to_string_lossy().to_string();
    let middle = middle.to_string_lossy().to_string();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create database with different timestamps
    let test_db_content = format!(
        "{}|5.0|{}\n{}|5.0|{}\n{}|5.0|{}\n",
        old,
        now - 1000,
        new,
        now,
        middle,
        now - 500
    );
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Test time-based sorting
    let output = Command::new(get_binary_path())
        .arg("-t")
        .arg("-l")
        .arg("path")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let list_output = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = list_output.lines().collect();

    // Should be sorted by time (most recent first)
    assert!(lines[0].contains(&new));
    assert!(lines[1].contains(&middle));
    assert!(lines[2].contains(&old));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_remove_directory() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let to_keep = temp_dir.path().join("to_keep");
    fs::create_dir(&to_keep).unwrap();
    let to_keep = to_keep.to_string_lossy().to_string();

    // Create database with test entries using current working directory
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let test_db_content = format!(
        "{}|5.0|1640995200\n{}|3.0|1640995100\n",
        current_dir, to_keep
    );
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Run remove (should remove current directory)
    let output = Command::new(get_binary_path())
        .arg("-x")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Verify it's removed
    let output = Command::new(get_binary_path())
        .arg("-l")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    let list_output = String::from_utf8(output.stdout).unwrap();
    assert!(!list_output.contains(&current_dir));
    assert!(list_output.contains(&to_keep));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_no_matches() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let projects = temp_dir.path().join("projects");
    fs::create_dir(&projects).unwrap();
    let projects = projects.to_string_lossy().to_string();

    // Create database with test entries
    let test_db_content = format!("{}|5.0|1640995200\n", projects);
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Search for non-existent pattern
    let output = Command::new(get_binary_path())
        .arg("-e")
        .arg("nonexistent")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    // Should fail (exit code != 0) when no matches found
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("no matches found"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_case_insensitive_matching() {
    let temp_data = create_temp_data_file();
    let temp_dir = tempdir().unwrap();
    let projects = temp_dir.path().join("Projects");
    fs::create_dir(&projects).unwrap();
    let projects = projects.to_string_lossy().to_string();

    // Create database with test entries
    let test_db_content = format!("{}|5.0|1640995200\n", projects);
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // Test case insensitive matching
    let output = Command::new(get_binary_path())
        .arg("-e")
        .arg("PROJ")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let match_output = String::from_utf8(output.stdout).unwrap();
    assert!(match_output.contains(&projects));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_empty_database() {
    let temp_data = create_temp_data_file();

    // Test with non-existent database file
    let output = Command::new(get_binary_path())
        .arg("-l")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let list_output = String::from_utf8(output.stdout).unwrap();
    assert!(list_output.trim().is_empty());
}

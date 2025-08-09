use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
        "/tmp/test_zjyo_integration_{}",
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
}

#[test]
fn test_add_and_list() {
    let temp_data = create_temp_data_file();

    // Create test database with known entry
    let test_db_content = "/tmp/test_dir|1.0|1640995200\n";
    fs::write(&temp_data, test_db_content).expect("Failed to write test data");

    // List directories
    let output = Command::new(get_binary_path())
        .arg("-l")
        .env("_Z_DATA", &temp_data)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let list_output = String::from_utf8(output.stdout).unwrap();
    assert!(list_output.contains("/tmp/test_dir"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_pattern_matching() {
    let temp_data = create_temp_data_file();

    // Manually create a test database file
    let test_db_content =
        "/home/user/projects|5.0|1640995200\n/home/user/documents|3.0|1640995100\n";
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
    assert!(match_output.contains("/home/user/projects"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_rank_sorting() {
    let temp_data = create_temp_data_file();

    // Create database with different ranks
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_db_content = format!(
        "/path/low|2.0|{}\n/path/high|10.0|{}\n/path/medium|5.0|{}\n",
        now, now, now
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
    assert!(lines[0].contains("/path/high"));
    assert!(lines[1].contains("/path/medium"));
    assert!(lines[2].contains("/path/low"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_time_sorting() {
    let temp_data = create_temp_data_file();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create database with different timestamps
    let test_db_content = format!(
        "/path/old|5.0|{}\n/path/new|5.0|{}\n/path/middle|5.0|{}\n",
        now - 1000,
        now,
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
    assert!(lines[0].contains("/path/new"));
    assert!(lines[1].contains("/path/middle"));
    assert!(lines[2].contains("/path/old"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_remove_directory() {
    let temp_data = create_temp_data_file();

    // Create database with test entries using current working directory
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let test_db_content = format!(
        "{}|5.0|1640995200\n/tmp/to_keep|3.0|1640995100\n",
        current_dir
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
    assert!(list_output.contains("/tmp/to_keep"));

    // Cleanup
    fs::remove_file(&temp_data).ok();
}

#[test]
fn test_no_matches() {
    let temp_data = create_temp_data_file();

    // Create database with test entries
    let test_db_content = "/home/user/projects|5.0|1640995200\n";
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

    // Create database with test entries
    let test_db_content = "/home/user/Projects|5.0|1640995200\n";
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
    assert!(match_output.contains("/home/user/Projects"));

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

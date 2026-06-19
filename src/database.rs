use crate::entry::DirEntry;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ZDatabase {
    pub entries: HashMap<String, DirEntry>,
    pub data_file: PathBuf,
}

impl Default for ZDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl ZDatabase {
    pub fn new() -> Self {
        let data_file = env::var("_Z_DATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home::home_dir().unwrap().join(".z"));

        let mut db = Self {
            entries: HashMap::new(),
            data_file,
        };
        db.load();
        db
    }

    pub fn load(&mut self) {
        if !self.data_file.exists() {
            return;
        }

        if let Ok(file) = File::open(&self.data_file) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.splitn(3, '|').collect();
                if parts.len() == 3 {
                    if let (Ok(rank), Ok(time)) = (parts[1].parse::<f64>(), parts[2].parse::<u64>())
                    {
                        if Path::new(parts[0]).is_dir() {
                            let entry = DirEntry::new(parts[0].to_string(), rank, time);
                            self.entries.insert(parts[0].to_string(), entry);
                        }
                    }
                }
            }
        }
    }

    pub fn save(&self) {
        let temp_file = self
            .data_file
            .with_extension(format!("tmp.{}", std::process::id()));

        if let Ok(mut file) = File::create(&temp_file) {
            for entry in self.entries.values() {
                writeln!(file, "{}|{}|{}", entry.path, entry.rank, entry.time).ok();
            }

            if file.sync_all().is_ok() {
                fs::rename(&temp_file, &self.data_file).ok();
            } else {
                fs::remove_file(&temp_file).ok();
            }
        }
    }

    pub fn add(&mut self, path: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(entry) = self.entries.get_mut(path) {
            entry.rank += 1.0;
            entry.time = now;
        } else {
            self.entries
                .insert(path.to_string(), DirEntry::new(path.to_string(), 1.0, now));
        }

        let total_rank: f64 = self.entries.values().map(|e| e.rank).sum();
        if total_rank > 9000.0 {
            self.entries.retain(|_, entry| {
                entry.rank *= 0.99;
                entry.rank >= 1.0
            });
        }

        self.save();
    }

    pub fn remove(&mut self, path: &str) {
        self.entries.remove(path);
        self.save();
    }

    pub fn find_matches(&self, pattern: &str, current_dir: Option<&str>) -> Vec<DirEntry> {
        let mut matches: Vec<DirEntry> = self
            .entries
            .values()
            .filter(|entry| {
                let path_lower = entry.path.to_lowercase();

                // Split pattern by whitespace and check that ALL words are contained in the path
                let pattern_words: Vec<&str> = pattern.split_whitespace().collect();
                let path_contains_all_words = if pattern_words.is_empty() {
                    true
                } else {
                    pattern_words
                        .iter()
                        .all(|word| path_lower.contains(&word.to_lowercase()))
                };

                if let Some(current) = current_dir {
                    path_lower.starts_with(&current.to_lowercase()) && path_contains_all_words
                } else {
                    path_contains_all_words
                }
            })
            .cloned()
            .collect();

        matches.sort_by_key(|b| std::cmp::Reverse(b.frecency()));
        matches
    }

    pub fn find_by_rank(&self, pattern: &str) -> Vec<DirEntry> {
        let mut matches: Vec<DirEntry> = self
            .entries
            .values()
            .filter(|entry| {
                let path_lower = entry.path.to_lowercase();
                let pattern_words: Vec<&str> = pattern.split_whitespace().collect();
                if pattern_words.is_empty() {
                    true
                } else {
                    pattern_words
                        .iter()
                        .all(|word| path_lower.contains(&word.to_lowercase()))
                }
            })
            .cloned()
            .collect();

        matches.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
        matches
    }

    pub fn find_by_time(&self, pattern: &str) -> Vec<DirEntry> {
        let mut matches: Vec<DirEntry> = self
            .entries
            .values()
            .filter(|entry| {
                let path_lower = entry.path.to_lowercase();
                let pattern_words: Vec<&str> = pattern.split_whitespace().collect();
                if pattern_words.is_empty() {
                    true
                } else {
                    pattern_words
                        .iter()
                        .all(|word| path_lower.contains(&word.to_lowercase()))
                }
            })
            .cloned()
            .collect();

        matches.sort_by_key(|b| std::cmp::Reverse(b.time));
        matches
    }
}

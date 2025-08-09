use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: String,
    pub rank: f64,
    pub time: u64,
}

impl DirEntry {
    pub fn new(path: String, rank: f64, time: u64) -> Self {
        Self { path, rank, time }
    }

    pub fn frecency(&self) -> i32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let dx = now - self.time;
        (10000.0 * self.rank * (3.75 / ((0.0001 * dx as f64 + 1.0) + 0.25))) as i32
    }
}

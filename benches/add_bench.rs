use std::env;
use std::time::Instant;
use tempfile::NamedTempFile;
use zjyo::ZDatabase;

const ITERATIONS: u32 = 1000;

fn bench_add(entry_count: usize) {
    let file = NamedTempFile::new().unwrap();
    env::set_var("_Z_DATA", file.path());

    let mut db = ZDatabase::new();
    for i in 0..entry_count {
        db.add(&format!("/tmp/bench-seed-{i}"));
    }

    let target = "/tmp/bench-seed-0";
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        db.add(target);
    }
    let elapsed = start.elapsed();

    println!(
        "{entry_count:>6} entries: {:>8.2}us/add  ({ITERATIONS} iterations, {:.2?} total)",
        elapsed.as_secs_f64() * 1_000_000.0 / ITERATIONS as f64,
        elapsed
    );
}

fn main() {
    println!("ZDatabase::add benchmark (includes full save-to-disk each call)\n");
    for entry_count in [10, 100, 500, 1000, 5000] {
        bench_add(entry_count);
    }
}

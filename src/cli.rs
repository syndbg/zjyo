use crate::database::ZDatabase;
use clap::{Arg, Command};
use std::env;
use std::path::Path;

pub fn run() {
    let matches = Command::new("zjyo")
        .about("jump around faster")
        .arg(
            Arg::new("pattern")
                .help("Directory pattern to match")
                .index(1),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .help("List matching directories")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("rank")
                .short('r')
                .help("Match by rank only")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .help("Match by recent access only")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("current")
                .short('c')
                .help("Restrict matches to subdirectories of current directory")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("echo")
                .short('e')
                .help("Echo the best match, don't cd to it")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exclude")
                .short('x')
                .help("Remove the current directory from the datafile")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("add")
                .long("add")
                .help("Add current directory to database")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let mut db = ZDatabase::new();

    if matches.get_flag("add") {
        if let Ok(current_dir) = env::current_dir() {
            db.add(&current_dir.to_string_lossy());
        }
        return;
    }

    if matches.get_flag("exclude") {
        if let Ok(current_dir) = env::current_dir() {
            db.remove(&current_dir.to_string_lossy());
        }
        return;
    }

    let empty_string = String::new();
    let pattern = matches
        .get_one::<String>("pattern")
        .unwrap_or(&empty_string);

    if pattern.is_empty() && !matches.get_flag("list") {
        eprintln!("Usage: zjyo [options] <pattern>");
        return;
    }

    let current_dir = if matches.get_flag("current") {
        env::current_dir()
            .ok()
            .map(|p| p.to_string_lossy().to_string())
    } else {
        None
    };

    let matching_dirs = if matches.get_flag("rank") {
        db.find_by_rank(pattern)
    } else if matches.get_flag("time") {
        db.find_by_time(pattern)
    } else {
        db.find_matches(pattern, current_dir.as_deref())
    };

    if matches.get_flag("list") {
        for entry in &matching_dirs {
            println!("{:<10} {:<10} {}", entry.frecency(), entry.rank, entry.path);
        }
        return;
    }

    if let Some(best_match) = matching_dirs.first() {
        if matches.get_flag("echo") {
            println!("{}", best_match.path);
        } else if Path::new(&best_match.path).exists() {
            println!("{}", best_match.path);
            db.add(&best_match.path);
        } else {
            db.remove(&best_match.path);
            eprintln!("z: directory no longer exists: {}", best_match.path);
            std::process::exit(1);
        }
    } else if !pattern.is_empty() {
        eprintln!("z: no matches found for: {}", pattern);
        std::process::exit(1);
    }
}

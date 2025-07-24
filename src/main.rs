use rocksdb::{DB, IteratorMode, Options};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read DB path from command-line argument
    let db_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rocksdb_cli <path/to/db> [--rw]");
        std::process::exit(1);
    });

    let mut read_write = false;

    if env::args().len() > 2 {
        let second_arg = env::args().nth(2).unwrap();
        if second_arg == "--rw" {
            read_write = true;
        }
    }

    let db;
    if !fs::exists(&db_path).unwrap() && !read_write {
        return Err(
            "Since the database doesn't exist yet, you must pass the --rw flag to open the database in read-write mode".into(),
        );
    } else if fs::exists(&db_path).unwrap() && !read_write {
        db = DB::open_for_read_only(&Options::default(), db_path.clone(), false)
            .expect("Failed to open DB in read-only mode");
    } else if read_write {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        db = DB::open(&opts, db_path).expect("Failed to open DB in read-write mode");
    } else {
        unreachable!()
    }

    loop {
        print!("rocksdb> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match parts.as_slice() {
            ["get", key] => match db.get(key) {
                Ok(Some(value)) => println!("{}", String::from_utf8_lossy(&value)),
                Ok(None) => println!("(not found)"),
                Err(e) => println!("Error: {}", e),
            },
            ["put", key, value] => {
                if !read_write {
                    println!(
                        "Error: Cannot put because the database was not opened in read-write mode"
                    );
                    continue;
                }
                if let Err(e) = db.put(key, value) {
                    println!("Error: {}", e);
                }
            }
            ["scan"] => {
                println!("All keys in the database:");
                for item in db.iterator(IteratorMode::Start) {
                    let (key, _value) = item?;
                    println!("{}", String::from_utf8_lossy(&key));
                }
            }
            ["scan", lim] => {
                let mut i = 0;
                let limit = lim.parse::<u64>().unwrap();
                println!("First {} keys in the database:", limit);
                for item in db.iterator(IteratorMode::Start) {
                    i += 1;
                    if i > limit {
                        break;
                    }
                    let (key, _value) = item?;
                    println!("{}", String::from_utf8_lossy(&key));
                }
            }
            ["count"] => {
                let mut i = 0;
                for item in db.iterator(IteratorMode::Start) {
                    item?;
                    i += 1;
                }
                println!("Total number of keys: {}", i);
            }
            ["exit"] => break Ok(()),
            ["q"] => break Ok(()),
            _ => println!("Usage: get <key> | put <key> <value> | scan [<limit>] | exit"),
        }
    }
}

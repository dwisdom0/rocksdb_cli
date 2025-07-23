use rocksdb::{DB, IteratorMode};
use std::io::{self, Write};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read DB path from command-line argument
    let db_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rocksdb_cli <path/to/db>");
        std::process::exit(1);
    });

    let db = DB::open_default(db_path).expect("Failed to open DB");

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
            ["count"] => {
                let mut i = 0;
                for item in db.iterator(IteratorMode::Start) {
                    item?;
                    i += 1;
                }
                println!("Total number of keys: {}", i);
            },
            ["exit"] => break Ok(()),
            ["q"] => break Ok(()),
            _ => println!("Usage: get <key> | put <key> <value> | scan | exit"),
        }
    }
}

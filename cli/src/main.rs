use std::io::{self, Write};
use cleaner::clean_url;

fn main() {
    println!("🧹 URL Cleaner - Paste your URL and press Enter");
    println!("(Press Ctrl+C to exit)");
    println!();
    
    loop {
        print!("Enter URL: ");
        io::stdout().flush().unwrap();
        
        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let url = input.trim();
                
                if url.is_empty() {
                    continue;
                }
                
                match clean_url(url) {
                    Ok(cleaned) => {
                        println!("Cleaned URL: {}", cleaned);
                        println!();
                    },
                    Err(e) => {
                        println!("Error: Invalid URL - {}", e);
                        println!();
                    },
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                println!();
            },
        }
    }
}


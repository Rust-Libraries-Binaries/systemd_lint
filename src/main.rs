use systemd_lint::{lint_service_file, suggest_optimizations};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "example.service"; // Change this to the path of your service file
    if let Ok(lines) = read_lines(path) {
        let content = lines.map(|l| l.unwrap()).collect::<Vec<String>>().join("\n");

        let warnings = lint_service_file(&content);
        let suggestions = suggest_optimizations(&content);

        println!("Warnings:");
        for warning in warnings {
            println!("- {}", warning);
        }

        println!("\nSuggestions:");
        for suggestion in suggestions {
            println!("- {}", suggestion);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

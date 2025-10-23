use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    // Get the command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <label>", args[0]);
        std::process::exit(1);
    }
    let label = &args[1];

    // Try to read tenant file
    let tenant = match std::fs::read_to_string("tenant") {
        Ok(content) => content.trim().to_string(),
        Err(_) => {
            eprintln!("Error: 'tenant' file not found. Please create a file named 'tenant' in the current directory.");
            std::process::exit(1);
        }
    };

    // Try to open ids file
    let ids_file = match File::open("ids") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: 'ids' file not found. Please create a file named 'ids' in the current directory.");
            std::process::exit(1);
        }
    };

    // Define the namespace UUID
    let namespace = Uuid::parse_str("8405ae4d-b315-42e1-918a-d1919900cf3f").expect("Invalid UUID");

    // Prepare output file
    let uuids_file = File::create("uuids")?;
    let reader = BufReader::new(ids_file);
    let mut writer = std::io::BufWriter::new(uuids_file);

    let mut counter = 0;

    for line in reader.lines() {
        let line = line?;
        let uuid_input = format!("{}:{}:{}", tenant, label, line);
        let uuid = Uuid::new_v5(&namespace, uuid_input.as_bytes());

        writeln!(writer, "{}", uuid)?;

        counter += 1;
        if counter % 10000 == 0 {
            print!("\rRecords processed: {}", counter);
            std::io::stdout().flush().unwrap();
        }
    }

    // Final overwrite message
    print!("\rTotal records processed: {}\n", counter);
    std::io::stdout().flush().unwrap();

    Ok(())
}


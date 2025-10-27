use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn sanitize_record(mut record: Vec<u8>) -> Vec<u8> {
    record.retain(|&b| b != 0x00); // Remove null bytes
    for byte in &mut record {
        if *byte == b'\r' || *byte == b'\n' || *byte == b'\t' || *byte == 0x08 {
            *byte = b' ';
        }
    }
    record
}

fn replace_invalid_utf8_bytes(input: &[u8]) -> (Vec<u8>, bool) {
    let mut output = Vec::with_capacity(input.len());
    let mut i = 0;
    let mut modified = false;

    while i < input.len() {
        match std::str::from_utf8(&input[i..]) {
            Ok(valid_str) => {
                output.extend_from_slice(valid_str.as_bytes());
                break;
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                output.extend_from_slice(&input[i..i + valid_up_to]);

                let error_len = e.error_len().unwrap_or(1);
                output.extend(std::iter::repeat(b' ').take(error_len));
                i += valid_up_to + error_len;
                modified = true;
            }
        }
    }

    (output, modified)
}

fn generate_output_filename(input_filename: &str) -> String {
    let path = Path::new(input_filename);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    if input_filename.ends_with(".mrc") {
        parent.join(format!("{}_fixedencoding.mrc", stem)).to_string_lossy().into_owned()
    } else {
        parent.join(format!("{}_fixedencoding.mrc", path.file_name().unwrap().to_string_lossy()))
            .to_string_lossy()
            .into_owned()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [marc_file]", args[0]);
        std::process::exit(1);
    }

    let input_filename = &args[1];
    let output_filename = generate_output_filename(input_filename);

    let input_data = fs::read(input_filename)?;
    let mut output_data = Vec::with_capacity(input_data.len());

    let mut record_start = 0;
    let mut total_records = 0;
    let mut modified_records = 0;
    let mut changed_to_a_count = 0;
    let mut last_reported = 0;
    let mut position9_counts: HashMap<u8, usize> = HashMap::new();

    for (i, &byte) in input_data.iter().enumerate() {
        if byte == 0x1D {
            total_records += 1;
            let mut record = input_data[record_start..=i].to_vec(); // include delimiter

            record = sanitize_record(record);

            if record.len() > 9 {
                let original_value = record[9];

                if original_value != b'a' && original_value != b' ' {
                    *position9_counts.entry(original_value).or_insert(0) += 1;
                    record[9] = b'a';
                    changed_to_a_count += 1;
                }

                if record[9] == b'a' {
                    let (replaced, was_modified) = replace_invalid_utf8_bytes(&record);
                    if was_modified {
                        record = replaced;
                        modified_records += 1;
                    }
                }
            }

            output_data.extend_from_slice(&record);
            record_start = i + 1;

            if total_records - last_reported >= 10_000 {
                print!("\rRecords processed: {} | Records modified: {}", total_records, modified_records);
                io::stdout().flush().unwrap();
                last_reported = total_records;
            }
        }
    }

    // Handle any trailing data after last delimiter
    if record_start < input_data.len() {
        total_records += 1;
        let mut record = input_data[record_start..].to_vec();

        record = sanitize_record(record);

        if record.len() > 9 {
            let original_value = record[9];

            if original_value != b'a' && original_value != b' ' {
                *position9_counts.entry(original_value).or_insert(0) += 1;
                record[9] = b'a';
                changed_to_a_count += 1;
            }

            if record[9] == b'a' {
                let (replaced, was_modified) = replace_invalid_utf8_bytes(&record);
                if was_modified {
                    record = replaced;
                    modified_records += 1;
                }
            }
        }

        output_data.extend_from_slice(&record);
    }

    fs::write(&output_filename, output_data)?;

    println!("\rRecords processed: {} | Records modified: {}\n", total_records, modified_records);

    if changed_to_a_count > 0 {
        println!("Records where record[9] was changed to 'a': {}", changed_to_a_count);
    }

    if !position9_counts.is_empty() {
        println!("\nOriginal values at position 9 that were changed to 'a':");
        for (byte, count) in position9_counts.iter() {
            println!("  0x{:02X} ('{}'): {}", byte, *byte as char, count);
        }
    }

    println!();
    Ok(())
}

use calamine::{open_workbook_auto, Reader, DataType};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn clean_cell(cell: &DataType, re: &Regex) -> String {
    match cell {
        DataType::String(s) => re.replace_all(s, " ").to_string(),
        DataType::Float(f) => f.to_string(),
        DataType::Int(i) => i.to_string(),
        DataType::Bool(b) => b.to_string(),
        _ => "".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [excel_file.xlsx]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = Path::new(input_path).with_extension("tsv");

    let mut workbook = open_workbook_auto(input_path).expect("Cannot open Excel file");
    let sheet_names = workbook.sheet_names().to_owned();
    let sheet_name = sheet_names.first().expect("No sheets found");

    print!("Initializing. Please be patient\n");

    let range = workbook.worksheet_range(sheet_name)
        .expect("Cannot read sheet")
        .expect("Invalid sheet range");

    let file = File::create(output_path.clone()).expect("Cannot create output file");
    let mut writer = BufWriter::new(file);

    let re = Regex::new(r"[\r\n]+").unwrap();
    let mut counter = 0;

    for row in range.rows() {
        let cleaned: Vec<String> = row.iter().map(|cell| clean_cell(cell, &re)).collect();
        writeln!(writer, "{}", cleaned.join("\t")).expect("Write failed");

        counter += 1;
        if counter % 1000 == 0 {
            print!("\rRecords processed: {}", counter);
            std::io::stdout().flush().unwrap();
        }
    }

    print!("\rTotal records processed: {}\n", counter);
}

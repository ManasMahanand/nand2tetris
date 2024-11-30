use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

pub mod code_writer;
pub mod parser;

fn main() {
    let file_names: Vec<String> = args().skip(1).collect();

    let (input_file_name, output_file_name) = if file_names.is_empty() {
        eprintln!("No file name provided!");
        std::process::exit(1);
    } else if file_names.len() == 1 {
        (
            &file_names[0],
            &format!("{}.asm", &file_names[0].split(".").next().unwrap()),
        )
    } else {
        (&file_names[0], &file_names[1])
    };

    let input_file = match File::open(input_file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("No file named {}!", input_file_name);
            std::process::exit(1);
        }
    };

    let output_file = match File::create_new(output_file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error creating output file!");
            std::process::exit(1);
        }
    };

    let input_file_reader = BufReader::new(input_file);
    let mut file_writer = BufWriter::new(output_file);

    for (i, line) in input_file_reader.lines().enumerate() {
        if line.is_err() {
            eprintln!("Error reading line at {i}!");
            std::process::exit(1);
        }
        let line = line.unwrap().trim().to_string();

        if line.is_empty() || &line[..2] == "//" {
            continue;
        }

        writeln!(&mut file_writer, "// {line}").unwrap();
        let command = parser::parse(line, i);
        code_writer::write(&mut file_writer, command, i);
    }
}

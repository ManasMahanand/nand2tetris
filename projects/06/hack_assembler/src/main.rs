use core::panic;
use std::collections::HashMap;
use std::io::Write;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

struct Symbol {
    addr: Vec<u16>,
    value: Option<u16>,
}

fn main() {
    let (input_file_name, output_file_name) = get_file_names_from_args();

    let input_file = File::open(input_file_name).expect("File does not exist");
    let output_file = File::create_new(output_file_name).expect("Output file already exists!");

    let file_reader = BufReader::new(input_file);
    let mut file_writer = BufWriter::new(output_file);

    let mut output_vec: Vec<Option<u16>> = Vec::new();
    let mut symbol_map: HashMap<String, Symbol> = HashMap::new();

    for (i, line) in file_reader.lines().enumerate() {
        let line = line.expect("Unable to read input file");
        let line = String::from(line.trim());

        if line.is_empty() || &line[..2] == "//" {
            continue;
        } else if &line[..1] == "@" {
            let num: Option<u16> = match Some(&line[1..].trim().parse()) {
                Some(Ok(num)) => Some(*num),
                _ => {
                    let symbol = String::from(line[1..].trim());
                    let some_num: Option<u16> = if &symbol[..1] == "R" {
                        let num: Option<u16> = match Some(symbol[1..].trim().parse::<u16>()) {
                            None => panic!("Impossible!"),
                            Some(Ok(num)) => Some(num),
                            Some(Err(_)) => match symbol_map.get_mut(symbol.as_str()) {
                                Some(sym) => {
                                    if sym.value.is_none() {
                                        sym.addr.push(
                                            <usize as TryInto<u16>>::try_into(output_vec.len())
                                                .unwrap(),
                                        );
                                    }
                                    sym.value
                                }
                                None => {
                                    symbol_map.insert(
                                        symbol,
                                        Symbol {
                                            addr: vec![<usize as TryInto<u16>>::try_into(
                                                output_vec.len(),
                                            )
                                            .unwrap()],
                                            value: None,
                                        },
                                    );
                                    None
                                }
                            },
                        };
                        if num > Some(15) {
                            panic!("Only registered within the range R0..R15 are accessible!");
                        }
                        num
                    } else {
                        match symbol.as_str() {
                            "SCREEN" => Some(16384),
                            "KBD" => Some(24567),
                            "SP" => Some(0),
                            "LCL" => Some(1),
                            "ARG" => Some(2),
                            "THIS" => Some(3),
                            "THAT" => Some(4),
                            _ => match symbol_map.get_mut(symbol.as_str()) {
                                Some(sym) => {
                                    if sym.value.is_none() {
                                        sym.addr.push(
                                            <usize as TryInto<u16>>::try_into(output_vec.len())
                                                .unwrap(),
                                        );
                                    }
                                    sym.value
                                }
                                None => {
                                    symbol_map.insert(
                                        symbol,
                                        Symbol {
                                            addr: vec![<usize as TryInto<u16>>::try_into(
                                                output_vec.len(),
                                            )
                                            .unwrap()],
                                            value: None,
                                        },
                                    );
                                    None
                                }
                            },
                        }
                    };
                    some_num
                }
            };
            output_vec.push(num);
        } else if &line[..1] == "(" {
            if &line[line.len() - 1..] != ")" {
                panic!("Unclosed parenthesis at {}", i + 1);
            }
            let label = String::from(&line[1..line.len() - 1]);
            match symbol_map.get_mut(label.as_str()) {
                Some(symbol) => {
                    if symbol.value.is_some() {
                        panic!("Error at line {}. Label already exists!", i + 1);
                    }
                    symbol.value =
                        Some(<usize as TryInto<u16>>::try_into(output_vec.len()).unwrap());

                    for addr in symbol.addr.clone() {
                        output_vec[<u16 as Into<usize>>::into(addr)] =
                            Some(<usize as TryInto<u16>>::try_into(output_vec.len()).unwrap());
                    }
                    symbol.addr.clear();
                }
                None => {
                    symbol_map.insert(
                        label,
                        Symbol {
                            addr: Vec::new(),
                            value: Some(
                                <usize as TryInto<u16>>::try_into(output_vec.len()).unwrap(),
                            ),
                        },
                    );
                }
            }
        } else {
            let instructions: Vec<&str> = line.split('=').collect();
            let output_dest: String;

            let mut output = String::from("111");

            let rhs: Vec<&str>;
            if instructions.len() == 1 {
                output_dest = String::from("000");
                rhs = instructions[0].split(';').collect();
            } else {
                output_dest = get_output_dest(instructions[0], i);
                rhs = instructions[1].split(';').collect();
            }

            let output_comp = get_output_comp(rhs[0], i);

            let output_jump: String = if rhs.len() > 1 {
                let input_jump = rhs[1];
                get_output_jump(input_jump, i)
            } else {
                String::from("000")
            };

            output.push_str(output_comp.as_str());
            output.push_str(output_dest.as_str());
            output.push_str(output_jump.as_str());

            let num: u16 = u16::from_str_radix(output.as_str(), 2).unwrap();
            output_vec.push(Some(num));
        }
    }

    let mut addr = 15;
    for (i, item) in output_vec.iter_mut().enumerate() {
        let item = match *item {
            Some(item) => item,
            None => {
                let mut item: u16 = 0;
                for (_, sym) in symbol_map.iter_mut() {
                    if sym.addr.contains(&(i as u16)) {
                        match sym.value {
                            Some(val) => {
                                item = val;
                                break;
                            }
                            None => {
                                addr += 1;
                                sym.value = Some(addr);
                                item = addr;
                                break;
                            }
                        };
                    }
                }
                item
            }
        };
        writeln!(file_writer, "{:016b}", item).expect("Error writing file");
    }
}

fn get_output_jump(input_jump: &str, i: usize) -> String {
    let input_jump = input_jump.trim();
    let mut output_jump = String::new();
    match input_jump {
        "null" => {
            output_jump.push_str("000");
        }
        "JGT" => {
            output_jump.push_str("001");
        }
        "JEQ" => {
            output_jump.push_str("010");
        }
        "JGE" => {
            output_jump.push_str("011");
        }
        "JLT" => {
            output_jump.push_str("100");
        }
        "JNE" => {
            output_jump.push_str("101");
        }
        "JLE" => {
            output_jump.push_str("110");
        }
        "JMP" => {
            output_jump.push_str("111");
        }
        _ => {
            panic!("Invalid syntax at line: {}", i + 1);
        }
    }
    output_jump
}

fn get_output_comp(input_comp: &str, line_num: usize) -> String {
    let input_comp = input_comp.trim();
    let i = line_num;
    let mut output_comp = String::new();
    match input_comp {
        "0" => {
            output_comp.push_str("0101010");
        }
        "1" => {
            output_comp.push_str("0111111");
        }
        "-1" => {
            output_comp.push_str("0111010");
        }
        "D" => {
            output_comp.push_str("0001100");
        }
        "A" | "M" => {
            let a = if input_comp == "A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("110000");
        }
        "!D" => {
            output_comp.push_str("0001101");
        }
        "!A" | "!M" => {
            let a = if input_comp == "!A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("110001");
        }
        "-D" => {
            output_comp.push_str("0001111");
        }
        "-A" | "-M" => {
            let a = if input_comp == "-A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("110011");
        }
        "D+1" => {
            output_comp.push_str("0011111");
        }
        "A+1" | "M+1" => {
            let a = if input_comp == "A+1" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("110111");
        }
        "D-1" => {
            output_comp.push_str("0001110");
        }
        "A-1" | "M-1" => {
            let a = if input_comp == "A-1" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("110010");
        }
        "D+A" | "D+M" => {
            let a = if input_comp == "D+A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("000010");
        }
        "D-A" | "D-M" => {
            let a = if input_comp == "D-A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("010011");
        }
        "A-D" | "M-D" => {
            let a = if input_comp == "A-D" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("000111");
        }
        "D&A" | "D&M" => {
            let a = if input_comp == "D&A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("000000");
        }
        "D|A" | "D|M" => {
            let a = if input_comp == "D|A" { '0' } else { '1' };
            output_comp.push(a);
            output_comp.push_str("010101");
        }
        _ => {
            panic!("Invalid syntax at line: {}", i + 1);
        }
    };
    output_comp
}

fn get_output_dest(input_dest: &str, line_num: usize) -> String {
    let input_dest = input_dest.trim();
    let i = line_num;
    let mut output_dest = String::new();
    match input_dest {
        "0" => {
            output_dest.push_str("000");
        }
        "M" => {
            output_dest.push_str("001");
        }
        "D" => {
            output_dest.push_str("010");
        }
        "DM" | "MD" => {
            output_dest.push_str("011");
        }
        "A" => {
            output_dest.push_str("100");
        }
        "AM" | "MA" => {
            output_dest.push_str("101");
        }
        "AD" | "DA" => {
            output_dest.push_str("110");
        }
        "ADM" | "AMD" | "DAM" | "DMA" | "MDA" | "MAD" => {
            output_dest.push_str("111");
        }
        _ => {
            panic!("Invalid syntax at line: {}", i + 1);
        }
    }
    output_dest
}

fn get_file_names_from_args() -> (String, String) {
    let file_names: Vec<String> = env::args().skip(1).collect();
    let mut output_file_name: String;

    if file_names.is_empty() {
        eprintln!("Expected at least 1 argument!");
        println!("Usage: hack_assembler <INPUT_FILE>");
        std::process::exit(1);
    } else if file_names.len() == 1 {
        output_file_name = file_names[0].clone().split('.').collect::<Vec<&str>>()[0].to_string();
        output_file_name += ".hack";
    } else {
        output_file_name = file_names[1].clone();
    }

    (file_names[0].clone(), output_file_name)
}

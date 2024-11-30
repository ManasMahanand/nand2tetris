const ARITHMETIC_COMMANDS: [&str; 9] = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

#[derive(Debug)]
pub enum Command {
    Arithmetic(ArithmeticType),
    PushPop(PushPopType),
}

#[derive(Debug)]
pub enum ArithmeticType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum PushPopType {
    Push(Segment, usize),
    Pop(Segment, usize),
}

#[derive(Debug)]
pub enum Segment {
    Local,
    Argument,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

pub fn parse(line: String, i: usize) -> Command {
    let words: Vec<&str> = line.split_whitespace().collect();

    if ARITHMETIC_COMMANDS.contains(&words[0]) {
        let arithmetic_command_type = get_arithetic_command_type(words[0], i);
        Command::Arithmetic(arithmetic_command_type)
    } else if words[0] == "push" || words[0] == "pop" {
        if words.len() != 3 {
            eprintln!("Invalid syntax at {i}");
            std::process::exit(1);
        }

        let segment = get_segment(words[1], i);
        let int = words[2].parse::<usize>();

        if int.is_err() {
            eprint!("Invalid syntax at {i}");
            std::process::exit(1);
        }
        let int = int.unwrap();

        let push_pop_type = get_push_pop_type(words[0], segment, int, i);

        Command::PushPop(push_pop_type)
    } else {
        eprintln!("Invalid syntax at {i}");
        std::process::exit(1);
    }
}

fn get_arithetic_command_type(arithmetic_command_string: &str, i: usize) -> ArithmeticType {
    match arithmetic_command_string {
        "add" => ArithmeticType::Add,
        "sub" => ArithmeticType::Sub,
        "neg" => ArithmeticType::Neg,
        "eq" => ArithmeticType::Eq,
        "gt" => ArithmeticType::Gt,
        "lt" => ArithmeticType::Lt,
        "and" => ArithmeticType::And,
        "or" => ArithmeticType::Or,
        "not" => ArithmeticType::Not,
        _ => {
            eprintln!("Invalid arithmetic command {arithmetic_command_string} at {i}!");
            std::process::exit(1);
        }
    }
}

fn get_push_pop_type(push_pop_string: &str, segment: Segment, int: usize, i: usize) -> PushPopType {
    match push_pop_string {
        "push" => PushPopType::Push(segment, int),
        "pop" => PushPopType::Pop(segment, int),
        _ => {
            eprintln!("Invalid push/pop command {push_pop_string} at {i}!");
            std::process::exit(1);
        }
    }
}

fn get_segment(segment_string: &str, i: usize) -> Segment {
    match segment_string {
        "local" => Segment::Local,
        "argument" => Segment::Argument,
        "static" => Segment::Static,
        "constant" => Segment::Constant,
        "this" => Segment::This,
        "that" => Segment::That,
        "pointer" => Segment::Pointer,
        "temp" => Segment::Temp,
        _ => {
            eprintln!("Invalid segment name {segment_string} at {i}!");
            std::process::exit(1);
        }
    }
}

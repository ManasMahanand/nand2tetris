use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::parser::{ArithmeticType, Command, Segment};

pub fn write(writer: &mut BufWriter<File>, command: Command, i: usize) {
    match command {
        Command::Arithmetic(arithmetic_command) => {
            write_arithmetic(arithmetic_command, writer, i);
        }
        Command::PushPop(push_pop_type) => match push_pop_type {
            crate::parser::PushPopType::Push(segment, i) => {
                write_push(segment, i, writer);
            }
            crate::parser::PushPopType::Pop(segment, i) => {
                write_pop(segment, i, writer);
            }
        },
    };
}

fn write_arithmetic(arithmetic_command: ArithmeticType, writer: &mut BufWriter<File>, i: usize) {
    match arithmetic_command {
        ArithmeticType::Add => {
            // @SP
            // M=M-1
            // A=M
            // D=M
            // A=A-1
            // M=M+D
            // @SP
            // M=M-1

            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "M=M+D").unwrap();
        }
        ArithmeticType::Sub => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "M=M-D").unwrap();
        }
        ArithmeticType::Neg => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=-M").unwrap();
        }
        ArithmeticType::Eq => {
            // @SP
            // M=M-1
            // A=M
            // D=M
            // A=A-1
            // D=M-D;
            // @EQ
            // D;JEQ
            // @SP
            // A=M-1
            // M=0
            // (EQ)
            // @SP
            // A=M-1
            // M=1

            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "D=M-D;").unwrap();
            writeln!(writer, "@EQ{i}").unwrap();
            writeln!(writer, "D;JEQ").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=0").unwrap();
            writeln!(writer, "@CONTINUE{i}").unwrap();
            writeln!(writer, "0;JMP").unwrap();
            writeln!(writer, "(EQ{i})").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=-1").unwrap();
            writeln!(writer, "(CONTINUE{i})").unwrap();
        }
        ArithmeticType::Gt => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "D=M-D;").unwrap();
            writeln!(writer, "@GT{i}").unwrap();
            writeln!(writer, "D;JGT").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=0").unwrap();
            writeln!(writer, "@CONTINUE{i}").unwrap();
            writeln!(writer, "0;JMP").unwrap();
            writeln!(writer, "(GT{i})").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=-1").unwrap();
            writeln!(writer, "(CONTINUE{i})").unwrap();
        }
        ArithmeticType::Lt => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "D=M-D;").unwrap();
            writeln!(writer, "@LT{i}").unwrap();
            writeln!(writer, "D;JLT").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=0").unwrap();
            writeln!(writer, "@CONTINUE{i}").unwrap();
            writeln!(writer, "0;JMP").unwrap();
            writeln!(writer, "(LT{i})").unwrap();
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=-1").unwrap();
            writeln!(writer, "(CONTINUE{i})").unwrap();
        }
        ArithmeticType::And => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "M=D&M;").unwrap();
            //            writeln!(writer, "@AND{i}").unwrap();
            //            writeln!(writer, "D;JGT").unwrap();
            //            writeln!(writer, "@SP").unwrap();
            //            writeln!(writer, "A=M-1").unwrap();
            //            writeln!(writer, "M=0").unwrap();
            //            writeln!(writer, "@CONTINUE{i}").unwrap();
            //            writeln!(writer, "0;JMP").unwrap();
            //            writeln!(writer, "(AND{i})").unwrap();
            //            writeln!(writer, "@SP").unwrap();
            //            writeln!(writer, "A=M-1").unwrap();
            //            writeln!(writer, "M=-1").unwrap();
            //            writeln!(writer, "(CONTINUE{i})").unwrap();
        }
        ArithmeticType::Or => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "M=M-1").unwrap();
            writeln!(writer, "A=M").unwrap();
            writeln!(writer, "D=M").unwrap();
            writeln!(writer, "A=A-1").unwrap();
            writeln!(writer, "M=M|D;").unwrap();
            //            writeln!(writer, "@OR{i}").unwrap();
            //            writeln!(writer, "D;JGT").unwrap();
            //            writeln!(writer, "@SP").unwrap();
            //            writeln!(writer, "A=M-1").unwrap();
            //            writeln!(writer, "M=0").unwrap();
            //            writeln!(writer, "@CONTINUE{i}").unwrap();
            //            writeln!(writer, "0;JMP").unwrap();
            //            writeln!(writer, "(OR{i})").unwrap();
            //            writeln!(writer, "@SP").unwrap();
            //            writeln!(writer, "A=M-1").unwrap();
            //            writeln!(writer, "M=-1").unwrap();
            //            writeln!(writer, "(CONTINUE{i})").unwrap();
        }
        ArithmeticType::Not => {
            writeln!(writer, "@SP").unwrap();
            writeln!(writer, "A=M-1").unwrap();
            writeln!(writer, "M=!M").unwrap();
        }
    }
}

fn write_push(segment: Segment, i: usize, writer: &mut BufWriter<File>) {
    let mut write_into_segment = |segment, i| {
        // @i
        // D=A
        //
        // @LCL
        // A=M+D
        // D=M
        //
        // @SP
        // A=M
        // M=D
        //
        // @SP
        // M=M+1

        writeln!(writer, "@{i}").unwrap();
        writeln!(writer, "D=A").unwrap();
        match segment {
            "constant" => {}
            _ => {
                writeln!(writer, "@{segment}").unwrap();
                writeln!(writer, "A=M+D").unwrap();
                writeln!(writer, "D=M").unwrap();
            }
        }
        writeln!(writer, "@SP").unwrap();
        writeln!(writer, "A=M").unwrap();
        writeln!(writer, "M=D").unwrap();
        writeln!(writer, "@SP").unwrap();
        writeln!(writer, "M=M+1").unwrap();
    };

    match segment {
        Segment::Local => {
            write_into_segment("LCL", i);
        }
        Segment::Argument => {
            write_into_segment("ARG", i);
        }
        Segment::Static => {
            write_into_segment("16", i);
        }
        Segment::Constant => {
            write_into_segment("constant", i);
        }
        Segment::This => {
            write_into_segment("THIS", i);
        }
        Segment::That => {
            write_into_segment("THAT", i);
        }
        Segment::Pointer => match i {
            0 => {
                writeln!(writer, "@THIS").unwrap();
                writeln!(writer, "D=M").unwrap();
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "A=M").unwrap();
                writeln!(writer, "M=D").unwrap();
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "M=M+1").unwrap();
            }
            1 => {
                writeln!(writer, "@THAT").unwrap();
                writeln!(writer, "D=M").unwrap();
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "A=M").unwrap();
                writeln!(writer, "M=D").unwrap();
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "M=M+1").unwrap();
            }
            _ => {}
        },
        Segment::Temp => {
            write_into_segment("5", i);
        }
    }
}

fn write_pop(segment: Segment, i: usize, writer: &mut BufWriter<File>) {
    let mut write_into_segment = |segment, i| {
        // @i
        // D=A
        // @LCL
        // M=M+D
        // @SP
        // M=M-1
        // A=M
        // D=M
        // @LCL
        // A=M
        // M=D
        // @i
        // D=A
        // @LCL
        // M=M-D
        writeln!(writer, "@{i}").unwrap();
        writeln!(writer, "D=A").unwrap();
        writeln!(writer, "@{segment}").unwrap();
        writeln!(writer, "M=M+D").unwrap();
        writeln!(writer, "@SP").unwrap();
        writeln!(writer, "M=M-1").unwrap();
        writeln!(writer, "A=M").unwrap();
        writeln!(writer, "D=M").unwrap();
        writeln!(writer, "@{segment}").unwrap();
        writeln!(writer, "A=M").unwrap();
        writeln!(writer, "M=D").unwrap();
        writeln!(writer, "@{i}").unwrap();
        writeln!(writer, "D=A").unwrap();
        writeln!(writer, "@{segment}").unwrap();
        writeln!(writer, "M=M-D").unwrap();
    };
    match segment {
        Segment::Local => {
            write_into_segment("LCL", i);
        }
        Segment::Argument => {
            write_into_segment("ARG", i);
        }
        Segment::Static => {
            write_into_segment("16", i);
        }
        Segment::Constant => {}
        Segment::This => {
            write_into_segment("THIS", i);
        }
        Segment::That => {
            write_into_segment("THAT", i);
        }
        Segment::Pointer => match i {
            0 => {
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "M=M-1").unwrap();
                writeln!(writer, "A=M").unwrap();
                writeln!(writer, "D=M").unwrap();
                writeln!(writer, "@THIS").unwrap();
                writeln!(writer, "M=D").unwrap();
            }
            1 => {
                writeln!(writer, "@SP").unwrap();
                writeln!(writer, "M=M-1").unwrap();
                writeln!(writer, "A=M").unwrap();
                writeln!(writer, "D=M").unwrap();
                writeln!(writer, "@THAT").unwrap();
                writeln!(writer, "M=D").unwrap();
            }
            _ => {}
        },
        Segment::Temp => {
            write_into_segment("5", i);
        }
    }
}

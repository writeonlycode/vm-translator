use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let (filename, _) = path.split_once(".").expect("Error removing extension.");
    let filename_label = filename.split("/").last().expect("Error parsing filename.");

    let program = fs::read_to_string(path).expect("Error reading file...");
    let result = translate(program.as_str(), filename_label);

    let filename = format!("{}.asm", filename);
    fs::write(filename, result).expect("Error writing file.");
}

fn translate(program: &str, filename: &str) -> String {
    let mut index = 0;
    let mut result = String::new();

    for line in program.lines() {
        if is_instruction(line) {
            result.push_str(translate_instruction(line, filename, &mut index).as_str());
        }
    }

    result
}

fn is_instruction(line: &str) -> bool {
    !line.starts_with("//") && !line.is_empty()
}

fn translate_instruction(line: &str, filename: &str, index: &mut u32) -> String {
    let instruction: Vec<&str> = line.split_whitespace().collect();

    if is_push_pop_instruction(&instruction) {
        translate_push_pop_instruction(&instruction, filename)
    } else if is_arithmetic_logical_instruction(&instruction) {
        translate_arithmetic_logical_instruction(&instruction, index)
    } else {
        String::new()
    }
}

fn is_push_pop_instruction(instruction: &Vec<&str>) -> bool {
    instruction.len() == 3
}

fn translate_push_pop_instruction(instruction: &Vec<&str>, filename: &str) -> String {
    if is_push_instruction(instruction) {
        translate_push_instruction(instruction, filename)
    } else {
        translate_pop_instruction(instruction, filename)
    }
}

fn is_push_instruction(instruction: &Vec<&str>) -> bool {
    instruction[0] == "push"
}

fn translate_push_instruction(instruction: &Vec<&str>, filename: &str) -> String {
    let segment = instruction[1];
    let index: u16 = match instruction[2].parse() {
        Ok(value) => value,
        Err(_) => 0,
    };

    let mut result = String::new();

    result.push_str(
        format!(
            "// Push value from {} at {} to the stack.\n",
            segment, index
        )
        .as_str(),
    );

    let segment = match segment {
        "local" => translate_push_local(index),
        "argument" => translate_push_argument(index),
        "this" => translate_push_this(index),
        "that" => translate_push_that(index),
        "pointer" => translate_push_pointer(index),
        "temp" => translate_push_temp(index),
        "constant" => translate_push_constant(index),
        "static" => translate_push_static(index, filename),
        _ => String::new(),
    };

    result.push_str(segment.as_str());
    result
}

fn translate_push_local(index: u16) -> String {
    format!(
        "@{}\nD=A\n@LCL\nA=M\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index
    )
}

fn translate_push_argument(index: u16) -> String {
    format!(
        "@{}\nD=A\n@ARG\nA=M\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index
    )
}

fn translate_push_this(index: u16) -> String {
    format!(
        "@{}\nD=A\n@THIS\nA=M\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index
    )
}

fn translate_push_that(index: u16) -> String {
    format!(
        "@{}\nD=A\n@THAT\nA=M\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index
    )
}

fn translate_push_pointer(index: u16) -> String {
    match index {
        0 => format!("@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"),
        1 => format!("@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"),
        _ => panic!(
            "Invalid index when pushing to stack from pointer: {}. Index should be 0 or 1.",
            index
        ),
    }
}

fn translate_push_temp(index: u16) -> String {
    if 1 <= index && index <= 7 {
        format!("@{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", 5 + index)
    } else {
        panic!(
            "Invalid index when pushing to stack from temporary: {}. Index should be between 1 and 7.",
            index
        )
    }
}

fn translate_push_constant(index: u16) -> String {
    format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index)
}

fn translate_push_static(index: u16, filename: &str) -> String {
    format!("@{}.{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", filename, index)
}

fn translate_pop_instruction(instruction: &Vec<&str>, filename: &str) -> String {
    let segment = instruction[1];
    let index: u16 = match instruction[2].parse() {
        Ok(value) => value,
        Err(_) => 0,
    };

    let mut result = String::new();

    result.push_str(format!("// Pop value from {} at {} to the stack.\n", segment, index).as_str());

    let segment = match segment {
        "local" => translate_pop_local(index),
        "argument" => translate_pop_argument(index),
        "this" => translate_pop_this(index),
        "that" => translate_pop_that(index),
        "pointer" => translate_pop_pointer(index),
        "temp" => translate_pop_temp(index),
        "static" => translate_pop_static(index, filename),
        _ => String::new(),
    };

    result.push_str(segment.as_str());
    result
}

fn translate_pop_local(index: u16) -> String {
    format!(
        "@{}\nD=A\n@LCL\nA=M\nD=D+A\n@R5\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R5\nA=M\nM=D\n",
        index
    )
}

fn translate_pop_argument(index: u16) -> String {
    format!(
        "@{}\nD=A\n@ARG\nA=M\nD=D+A\n@R5\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R5\nA=M\nM=D\n",
        index
    )
}

fn translate_pop_this(index: u16) -> String {
    format!(
        "@{}\nD=A\n@THIS\nA=M\nD=D+A\n@R5\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R5\nA=M\nM=D\n",
        index
    )
}

fn translate_pop_that(index: u16) -> String {
    format!(
        "@{}\nD=A\n@THAT\nA=M\nD=D+A\n@R5\nM=D\n@SP\nM=M-1\n@SP\nA=M\nD=M\n@R5\nA=M\nM=D\n",
        index
    )
}

fn translate_pop_pointer(index: u16) -> String {
    match index {
        0 => format!("@SP\nM=M-1\nA=M\nD=M\n@THIS\nM=D\n"),
        1 => format!("@SP\nM=M-1\nA=M\nD=M\n@THAT\nM=D\n"),
        _ => panic!(
            "Invalid index when poping from the stack to pointer: {}. Index should be 0 or 1.",
            index
        ),
    }
}

fn translate_pop_temp(index: u16) -> String {
    if 1 <= index && index <= 7 {
        format!("@SP\nM=M-1\nA=M\nD=M\n@{}\nM=D\n", 5 + index)
    } else {
        panic!(
            "Invalid index when popping from the stack to temporary: {}. Index should be between 1 and 7.",
            index
        )
    }
}

fn translate_pop_static(index: u16, filename: &str) -> String {
    format!("@SP\nM=M-1\nA=M\nD=M\n@{}.{}\nM=D\n", filename, index)
}

fn is_arithmetic_logical_instruction(instruction: &Vec<&str>) -> bool {
    instruction.len() == 1
}

fn translate_arithmetic_logical_instruction(instruction: &Vec<&str>, index: &mut u32) -> String {
    let instruction = instruction[0];

    let mut result = String::new();

    result.push_str(format!("// Arithmetic instruction: {}.\n", instruction).as_str());

    let instruction = match instruction {
        "add" => translate_add_instruction(),
        "sub" => translate_sub_instruction(),
        "neg" => translate_neg_instruction(),
        "eq" => translate_eq_instruction(index),
        "gt" => translate_gt_instruction(index),
        "lt" => translate_lt_instruction(index),
        "and" => translate_and_instruction(),
        "or" => translate_or_instruction(),
        "not" => translate_not_instruction(),
        _ => panic!(
            "Arithmetic or logical operation not recognized: {}",
            instruction
        ),
    };

    result.push_str(instruction.as_str());
    result
}

fn translate_add_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D+M\n@SP\nM=M+1")
}

fn translate_sub_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=M-D\n@SP\nM=M+1")
}

fn translate_neg_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nM=-M\n@SP\nM=M+1")
}

fn translate_eq_instruction(index: &mut u32) -> String {
    *index += 1;

    format!(
        "@SP\nM=M-1\nD=M\n@R6\nM=D\n@SP\nM=M-1\nD=M\n@R5\nM=D\n@R5\nA=M\nD=M\n@R6\nA=M\nD=M-D\n@RESULT_TRUE_{}\nD;JEQ\n(RESULT_FALSE_{})\n@R7\nM=0\n@RESULT_{}\n0;JMP\n(RESULT_TRUE_{})\n@R7\nM=-1\n(RESULT_{})\n@R7\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index, index, index, index, index
    )
}

fn translate_gt_instruction(index: &mut u32) -> String {
    *index += 1;

    format!(
        "@SP\nM=M-1\nD=M\n@R6\nM=D\n@SP\nM=M-1\nD=M\n@R5\nM=D\n@R5\nA=M\nD=M\n@R6\nA=M\nD=D-M\n@RESULT_TRUE_{}\nD;JGT\n(RESULT_FALSE_{})\n@R7\nM=0\n@RESULT_{}\n0;JMP\n(RESULT_TRUE_{})\n@R7\nM=-1\n(RESULT_{})\n@R7\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
        index, index, index, index, index
    )
}

fn translate_lt_instruction(index: &mut u32) -> String {
    *index += 1;

    format!(
        "@SP\nM=M-1\nD=M\n@R6\nM=D\n@SP\nM=M-1\nD=M\n@R5\nM=D\n@R5\nA=M\nD=M\n@R6\nA=M\nD=D-M\n@RESULT_TRUE_{}\nD;JLT\n(RESULT_FALSE_{})\n@R7\nM=0\n@RESULT_{}\n0;JMP\n(RESULT_TRUE_{})\n@R7\nM=-1\n(RESULT_{})\n@R7\nD=M\n@SP\nA=M \nM=D\n@SP\nM=M+1\n",
        index, index, index, index, index
    )
}

fn translate_and_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D&M\n@SP\nM=M+1")
}

fn translate_or_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nM=D|M\n@SP\nM=M+1")
}

fn translate_not_instruction() -> String {
    format!("@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1")
}

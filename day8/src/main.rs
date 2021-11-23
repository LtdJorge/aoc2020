#[derive(Debug)]
enum Instruction {
    NOP,
    ACC,
    JMP,
    ERR,
}

fn process_instruction(
    instruction_call: &(Instruction, i32),
    stack: &Vec<i32>,
    function_pointer: &Vec<i32>,
) -> (Vec<i32>, Vec<i32>) {
    let mut new_stack = stack.clone();
    let mut new_fp = function_pointer.clone();
    match instruction_call.0 {
        Instruction::NOP => {
            // println!("----NOP instruction----");
            // println!("Stack: {:?}", stack);
            // println!("Function pointer: {:?}", function_pointer);
            // println!("Instruction: {:?} {:?}", instruction_call.0, instruction_call.1);
            new_fp.push(new_fp[new_fp.len() - 1] + 1);
            // println!("New stack: {:?}", new_stack);
            // println!("New function pointer: {:?}", new_fp);
            // println!("-----------------------");
        }
        Instruction::ACC => {
            // println!("----ACC instruction----");
            // println!("Stack: {:?}", stack);
            // println!("Function pointer: {:?}", function_pointer);
            // println!("Instruction: {:?} {:?}", instruction_call.0, instruction_call.1);
            new_stack.push(new_stack[new_stack.len() - 1] + instruction_call.1);
            new_fp.push(new_fp[new_fp.len() - 1] + 1);
            // println!("New stack: {:?}", new_stack);
            // println!("New function pointer: {:?}", new_fp);
            // println!("-----------------------");
        }
        Instruction::JMP => {
            //Check if already visited
            // println!("----JMP instruction----");
            // println!("Stack: {:?}", stack);
            // println!("Function pointer: {:?}", function_pointer);
            // println!("Instruction: {:?} {:?}", instruction_call.0, instruction_call.1);
            new_fp.push(new_fp[new_fp.len() - 1] + instruction_call.1);
            // println!("New stack: {:?}", new_stack);
            // println!("New function pointer: {:?}", new_fp);
            // println!("-----------------------");
        }
        _ => {}
    }
    return (new_stack, new_fp);
}

fn parse_instruction(instruction: String) -> (Instruction, i32) {
    let mut _instruction_pair: Vec<_> = instruction.split_whitespace().collect();
    let instruction_inst: Instruction = match _instruction_pair[0].to_uppercase().as_str() {
        "NOP" => Instruction::NOP,
        "ACC" => Instruction::ACC,
        "JMP" => Instruction::JMP,
        _ => {
            println!("Error parsing instruction {}", instruction);
            Instruction::ERR
        }
    };
    let instruction_val: i32 = match _instruction_pair[1] {
        "" => 0,
        _string_number => _string_number.to_string().parse::<i32>().unwrap(),
    };
    return (instruction_inst, instruction_val);
}

fn check_instruction_visited(pointer: i32, function_pointer: &Vec<i32>) -> bool {
    let new_function_pointer = &function_pointer[0..function_pointer.len() - 1];
    if new_function_pointer.contains(&pointer) {
        true
    } else {
        false
    }
}

fn main() {
    let fake_instruction_list = [
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    let mut parsed_instructions_list: Vec<(Instruction, i32)> = vec![];
    for instruction in fake_instruction_list {
        let parsed_instruction = parse_instruction(instruction.to_string());
        // println!(
        //     "Parsed instruction: ({:?},{:?})",
        //     parsed_instruction.0, parsed_instruction.1
        // );
        parsed_instructions_list.push(parsed_instruction);
    }

    let mut stack: Vec<i32> = vec![0];
    let mut function_pointer: Vec<i32> = vec![0];

    // For (range 0 to len - 1), parse instruction in position given by function_pointer
    for _index in 0..parsed_instructions_list.len() - 1 {
        let last_instruction_pointer = function_pointer.last().unwrap().to_owned();
        if _index != 0 && check_instruction_visited(last_instruction_pointer, &function_pointer) {
            println!(
                "Found repeated instruction '{:?}'. The last stack value is: {:?}",
                parsed_instructions_list[last_instruction_pointer as usize], stack.last().unwrap()
            );
            return;
        }
        let val = process_instruction(
            parsed_instructions_list
                .get(last_instruction_pointer as usize)
                .unwrap(),
            &stack,
            &function_pointer,
        );
        stack = val.0;
        function_pointer = val.1;
    }
}

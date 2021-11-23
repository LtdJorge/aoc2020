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
    let _fake_instruction_list = [
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    let instruction_list = [
        "acc +14", "acc +11", "nop +422", "acc +14", "jmp +443", "acc +21", "nop +524", "acc -2",
        "jmp +279", "jmp +1", "acc +28", "acc +11", "jmp +576", "acc +32", "acc -12", "acc -8",
        "jmp +291", "nop +542", "acc +41", "jmp +320", "acc +40", "jmp +96", "jmp +85", "acc +38",
        "acc +8", "jmp +333", "acc +44", "nop +231", "acc +40", "jmp +323", "acc +18", "jmp +251",
        "acc -1", "jmp +385", "acc -9", "acc +48", "acc +20", "acc +34", "jmp +150", "nop +203",
        "acc +4", "acc +32", "acc +44", "jmp +168", "acc +26", "acc +46", "acc +40", "jmp -30",
        "jmp +182", "acc +18", "jmp +404", "nop +142", "jmp +84", "acc +30", "acc +10", "jmp +1",
        "acc +40", "jmp +370", "jmp +381", "jmp +239", "acc -2", "acc +47", "acc -4", "jmp +295",
        "jmp -38", "acc +40", "acc +44", "acc +4", "acc +4", "jmp +156", "acc +31", "acc +20",
        "acc +0", "acc -12", "jmp -48", "acc +32", "acc +38", "jmp +1", "acc -6", "jmp +375",
        "acc +33", "acc +27", "acc +28", "jmp +107", "acc +1", "acc +6", "nop +136", "jmp +85",
        "acc +31", "acc +49", "acc +46", "jmp +167", "acc +5", "acc -5", "jmp +148", "acc +22",
        "acc +44", "acc -8", "acc -2", "jmp -60", "nop +354", "jmp +59", "acc +48", "nop +473",
        "acc -7", "acc +4", "jmp +105", "jmp +456", "acc +16", "acc +33", "acc +24", "jmp -4",
        "acc +36", "acc +10", "nop +441", "jmp +268", "jmp +388", "acc +0", "acc +27", "acc -1",
        "jmp -60", "nop +90", "jmp -90", "acc +48", "acc +30", "jmp +284", "acc +4", "acc +6",
        "acc +1", "acc -10", "jmp +95", "acc +35", "jmp +235", "acc +31", "acc -19", "jmp -96",
        "jmp +326", "acc -7", "acc +0", "acc -1", "jmp +53", "acc +15", "acc -14", "jmp +450",
        "nop +8", "acc -2", "acc -1", "acc +17", "jmp -25", "nop +444", "jmp +65", "jmp -86",
        "acc +44", "acc +16", "acc +32", "acc -11", "jmp +32", "acc +14", "acc +28", "jmp +123",
        "jmp +127", "jmp -44", "acc +42", "acc +24", "acc -3", "acc +4", "jmp +219", "acc +28",
        "acc +30", "acc -14", "acc -11", "jmp +67", "acc +5", "acc +43", "acc +23", "nop +73",
        "jmp +176", "acc +28", "acc +8", "acc +42", "acc +44", "jmp +278", "acc +9", "acc +46",
        "acc +0", "acc +30", "jmp +72", "jmp +317", "jmp +352", "jmp +273", "jmp +137", "nop +364",
        "jmp +249", "nop +79", "jmp +1", "jmp -147", "acc -10", "acc -1", "acc +12", "acc +27",
        "jmp +147", "acc -5", "acc +7", "jmp +63", "acc +33", "acc +32", "nop +81", "jmp -185",
        "acc +44", "jmp +215", "jmp +187", "acc +14", "acc +38", "jmp -113", "jmp +267", "acc -9",
        "acc +21", "acc -5", "jmp +143", "nop -57", "nop +281", "jmp -170", "jmp +267", "nop -131",
        "jmp -83", "acc -6", "jmp -95", "acc -9", "acc -8", "jmp +184", "acc +32", "acc -16",
        "jmp +171", "acc +5", "acc +22", "acc -7", "acc +20", "jmp +45", "acc +48", "jmp +239",
        "acc -4", "jmp +75", "acc -18", "jmp -178", "nop +349", "acc -12", "nop +313", "jmp -57",
        "acc +7", "acc +6", "jmp -241", "acc +19", "jmp +320", "acc +13", "jmp -61", "acc +0",
        "nop +337", "jmp +66", "acc +27", "acc -11", "acc -7", "jmp +315", "acc +23", "acc +26",
        "acc -5", "jmp +132", "acc +45", "acc +21", "acc -12", "jmp +158", "acc +19", "jmp +176",
        "acc +43", "jmp +124", "nop +227", "nop -236", "acc +11", "jmp +1", "jmp -67", "acc +21",
        "jmp +161", "jmp +86", "acc +26", "acc +7", "jmp +246", "acc +0", "jmp +215", "jmp +1",
        "acc +16", "jmp -257", "acc +2", "jmp +281", "nop -10", "acc +46", "jmp +124", "acc +13",
        "acc +24", "jmp +204", "jmp +1", "acc +23", "jmp +225", "nop -243", "jmp +167", "jmp +1",
        "jmp -142", "acc -15", "jmp -113", "acc +27", "acc -18", "acc +12", "jmp -259", "nop +74",
        "acc +35", "acc +42", "acc -4", "jmp -166", "nop +87", "nop +86", "acc +18", "acc -2",
        "jmp +212", "acc -8", "jmp -313", "acc +36", "acc -11", "jmp -233", "jmp +237", "nop +67",
        "acc +16", "nop -57", "jmp -92", "acc +48", "acc +2", "acc +21", "jmp +33", "acc -15",
        "jmp +145", "acc +26", "jmp -254", "acc +30", "acc +4", "acc -1", "acc -14", "jmp -64",
        "acc +32", "acc +8", "jmp -131", "acc -13", "jmp +138", "acc +5", "acc +4", "jmp -4",
        "acc +37", "nop -278", "acc +28", "acc +17", "jmp -215", "jmp -104", "nop -241", "jmp -43",
        "jmp -2", "acc +5", "acc -1", "jmp +151", "jmp +1", "acc +21", "jmp +19", "acc +40",
        "jmp +91", "acc +50", "nop +202", "acc -12", "jmp -333", "nop -66", "acc +42", "acc +7",
        "jmp +1", "jmp +47", "acc +32", "acc +29", "acc +42", "nop -8", "jmp +52", "jmp -299",
        "jmp +40", "acc +36", "acc -5", "acc +39", "jmp -116", "acc +19", "acc +30", "acc +39",
        "acc -1", "jmp -276", "jmp -245", "acc +6", "jmp -185", "acc +50", "acc +14", "acc -7",
        "jmp -325", "acc +33", "jmp -279", "nop +173", "acc +15", "acc -17", "jmp -33", "acc +20",
        "jmp -101", "acc -17", "jmp -335", "nop -8", "jmp +22", "acc +0", "acc +4", "jmp -133",
        "nop -81", "jmp +64", "jmp -306", "acc -19", "acc +31", "acc +47", "acc +26", "jmp +55",
        "jmp -402", "acc +13", "jmp -375", "acc +6", "acc -1", "acc -6", "acc +49", "jmp -28",
        "acc -7", "jmp -203", "jmp -395", "acc +5", "acc +38", "acc +10", "jmp +130", "jmp +161",
        "jmp -382", "acc +45", "jmp +113", "acc +38", "acc +48", "acc +46", "jmp +126", "acc -1",
        "acc -10", "acc +4", "acc +2", "jmp -425", "acc +0", "jmp -80", "acc +4", "jmp -202",
        "acc +25", "acc +8", "jmp -398", "jmp -307", "acc +3", "jmp +17", "acc +13", "acc +33",
        "acc +7", "jmp -381", "acc +5", "acc +12", "jmp -308", "jmp +1", "acc +3", "acc -14",
        "acc +46", "jmp -415", "acc +31", "acc +7", "acc +28", "jmp -419", "jmp -175", "jmp +1",
        "jmp -141", "acc +20", "nop -35", "jmp -36", "acc -6", "jmp +108", "nop +1", "jmp +8",
        "jmp -49", "jmp -389", "acc +24", "nop -482", "acc +41", "acc +25", "jmp -167", "nop -26",
        "jmp -198", "nop -199", "acc +23", "acc -19", "jmp -202", "jmp +58", "acc +3", "jmp -237",
        "acc +44", "acc +42", "acc +22", "acc +5", "jmp -307", "acc +45", "nop -418", "acc +41",
        "nop -88", "jmp +63", "acc +12", "nop -56", "acc -19", "jmp +55", "acc -13", "acc -7",
        "jmp -213", "acc +42", "jmp -88", "acc +20", "jmp -115", "acc +6", "jmp -57", "acc +25",
        "acc +49", "jmp -43", "jmp -322", "jmp -456", "acc +7", "acc +40", "acc +35", "jmp -518",
        "nop -461", "acc +43", "acc +33", "jmp +7", "acc +27", "jmp +5", "acc -15", "acc -19",
        "acc -2", "jmp -238", "acc +49", "acc +48", "acc -16", "jmp +34", "acc -6", "acc +49",
        "acc -4", "acc +4", "jmp +1", "acc +35", "nop -264", "jmp -234", "jmp -365", "jmp -436",
        "acc +20", "acc +36", "jmp -426", "acc +39", "acc +20", "jmp -343", "nop -443", "jmp -325",
        "jmp -127", "nop -560", "acc +10", "jmp -511", "jmp -455", "acc -16", "acc +18", "jmp -61",
        "acc +26", "jmp -285", "jmp +1", "nop -397", "acc +12", "nop -67", "jmp -371", "acc +27",
        "acc +13", "jmp -395", "acc +44", "jmp -565", "acc +1", "jmp -21", "nop -428", "acc -4",
        "jmp -265", "acc +48", "acc +10", "acc +46", "jmp -202", "acc -4", "acc -10", "jmp -152",
        "acc +17", "acc -10", "acc +22", "acc +10", "jmp +1",
    ];

    let mut parsed_instructions_list: Vec<(Instruction, i32)> = vec![];
    for instruction in instruction_list {
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
                parsed_instructions_list[last_instruction_pointer as usize],
                stack.last().unwrap()
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

#[derive(Debug, Clone)]
enum Instruction {
    Copy { from: String, to: char },
    Inc { to: char },
    Dec { to: char },
    Jump { x: String, offset: i32 },
    None
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let lines = input.trim();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines.lines() {
        instructions.push(parse_instruction(line));
    }

    instructions
}

fn parse_instruction(input: &str) -> Instruction {
    let instruction = input.trim();
    let parts: Vec<&str> = instruction.split(' ').collect();
    match parts[0] {
        "cpy" => Instruction::Copy { from: String::from(parts[1]), to: parts[2].chars().nth(0).unwrap() },
        "inc" => Instruction::Inc { to: parts[1].chars().nth(0).unwrap() },
        "dec" => Instruction::Dec { to: parts[1].chars().nth(0).unwrap() },
        "jnz" => Instruction::Jump { x: String::from(parts[1]), offset: parts[2].parse::<i32>().unwrap() },
        &_ => Instruction::None
    }
}

fn execute_instruction(instruction: &Instruction, w: i32, x: i32, y: i32, z: i32) -> (i32, i32, i32, i32) {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (w, x, y, z);
    match instruction {
        Instruction::Copy { from, to } => {
            let int_value: i32 = match from.parse() {
                Ok(x) => x,
                Err(_) => {
                    match from.chars().nth(0).unwrap() {
                        'a' => a,
                        'b' => b,
                        'c' => c,
                        'd' => d,
                        _ => 0,
                    }
                }
            };
            match to {
                'a' => a += int_value,
                'b' => b += int_value,
                'c' => c += int_value,
                'd' => d += int_value,
                _ => {}
            };
            (a, b, c, d)
        },
        Instruction::Inc { to } => {
            match to {
                'a' => a += 1,
                'b' => a += 1,
                'c' => a += 1,
                'd' => a += 1,
                _ => {}
            };
            (a, b, c, d)
        },
        Instruction::Dec { to } => {
            match to {
                'a' => a -= 1,
                'b' => a -= 1,
                'c' => a -= 1,
                'd' => a -= 1,
                _ => {}
            };
            (a, b, c, d)
        },
        Instruction::Jump { x, offset } => {(a, b, c, d)},
        Instruction::None => {(a, b, c, d)}
    }
}

fn execute_program(instructions: Vec<Instruction>) {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (0, 0, 0, 0);
    for instruction in instructions {
        match instruction {
            Instruction::Copy { from, to } => {
                let result = execute_instruction(instruction, a, b, c, d);
                a = result.0;
                b = result.1;
                c = result.2;
                d = result.3;
            },
            Instruction::Inc { to } => {
                let result = execute_instruction(instruction, a, b, c, d);
                a = result.0;
                b = result.1;
                c = result.2;
                d = result.3;
            },
            Instruction::Dec { to } => {
                let result = execute_instruction(instruction, a, b, c, d);
                a = result.0;
                b = result.1;
                c = result.2;
                d = result.3;
            },
            Instruction::Jump { x, offset } => {},
            Instruction::None => {}
        }
    }
    println!("a, b, c, d: {}, {}, {}, {}", a, b, c, d);
}

fn main() {
    execute_program(parse_instructions("cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5
"));
}
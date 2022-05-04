use clap::Parser;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(args.input_file)?;

    match interpret(file_contents, args.cells) {
        Ok(_) => Ok(()),
        Err(e) => Err((match e.error_type {
            BfErrorType::UnmatchedBrackets => "unmatched brackets".to_string(),
        } + format![" at {}", e.location].as_str())
        .into()),
    }
}

fn interpret(program: String, cells_size: usize) -> Result<(), BfError> {
    check_brackets(program.clone())?;

    let mut cells: Vec<u8> = vec![0; cells_size];
    let mut pointer: usize = 0;
    let mut program_counter: usize = 0;

    while program_counter < program.len() {
        let c = program.chars().nth(program_counter).unwrap();

        match c {
            '>' => {
                if pointer == cells.len() - 1 {
                    pointer = 0;
                }

                pointer += 1;
            }
            '<' => {
                if pointer == 0 {
                    pointer = cells.len() - 1;
                }

                pointer -= 1;
            }
            '+' => {
                if cells[pointer] == 255 {
                    cells[pointer] = 0;
                } else {
                    cells[pointer] += 1;
                }
            }
            '-' => {
                if cells[pointer] == 0 {
                    cells[pointer] = 255;
                } else {
                    cells[pointer] -= 1;
                }
            }
            '.' => {
                print!("{}", cells[pointer] as char);
            }
            ',' => {
                cells[pointer] = std::io::stdin().bytes().next().unwrap().unwrap();
            }
            '[' => {
                if cells[pointer] == 0 {
                    // skip to the end of the loop
                    while program.chars().nth(program_counter).unwrap() != ']' {
                        program_counter += 1;
                    }
                }
            }
            ']' => {
                if cells[pointer] != 0 {
                    // back to the start of the loop
                    while program.chars().nth(program_counter).unwrap() != '[' {
                        program_counter -= 1;
                    }
                }
            }
            _ => {}
        }

        program_counter += 1;
    }

    Ok(())
}

fn check_brackets(program: String) -> Result<(), BfError> {
    let mut stack = vec![];

    for (i, c) in program.chars().enumerate() {
        match c {
            '[' => {
                stack.push(i);
            }
            ']' => {
                if stack.is_empty() {
                    return Err(BfError::new(i, BfErrorType::UnmatchedBrackets));
                }
                stack.pop();
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err(BfError::new(
            *stack.last().unwrap(),
            BfErrorType::UnmatchedBrackets,
        ));
    }

    Ok(())
}

struct BfError {
    location: usize,
    error_type: BfErrorType,
}

impl BfError {
    fn new(location: usize, error_type: BfErrorType) -> BfError {
        BfError {
            location,
            error_type,
        }
    }
}

enum BfErrorType {
    UnmatchedBrackets,
}

#[derive(Parser)]
#[clap(version)]
struct Args {
    input_file: String,
    #[clap(short, long, default_value_t = 30_000)]
    cells: usize,
}

use std::io::{stdin, stdout, Write};

use casio::{compiler, emulator, g3a};

use chrono::Utc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Rizm {
    Compile {
        #[structopt(short = "i")]
        input: String,
        #[structopt(short = "o")]
        output: String,
    },
    Emulate {
        #[structopt(short = "i")]
        input: String,
    },
}

fn main() {
    let cmd = Rizm::from_args();

    match cmd {
        Rizm::Compile { input, output } => {
            // Actually compiling a program
            let compiler_content = std::fs::read_to_string(input).unwrap();
            let compiled_code = compiler::compile(&compiler_content);

            let mut compiled_file_builder =
                g3a::FileBuilder::new("test".to_string(), Utc::now().naive_utc());
            compiled_file_builder
                .short_name("test".to_string())
                .internal_name("@TEST".to_string())
                .code(compiled_code);
            let compiled_file = compiled_file_builder.finish();

            let output_path = std::path::Path::new(&output);
            let output_name = output_path.file_name().unwrap();
            std::fs::write(
                output_path,
                compiled_file.serialize(&format!("/{}", output_name.to_str().unwrap())),
            )
            .unwrap();
        }
        Rizm::Emulate { input } => {
            let raw_file = std::fs::read(input).unwrap();
            let file = g3a::File::parse(&raw_file).unwrap();

            let mut em = emulator::Emulator::new(file);
            loop {
                let mut cli_in = String::new();
                stdout().write(&[b'>']).expect("Writing to Stdout");
                stdout().flush().expect("Flushing StdOut");
                stdin()
                    .read_line(&mut cli_in)
                    .expect("Could not get string");
                cli_in.remove(cli_in.len() - 1);

                let mut em_cmd = cli_in.split(" ");
                match em_cmd.next() {
                    Some("run") => loop {
                        if let Err(e) = em.emulate_single() {
                            println!("Error: {:?}", e);
                            break;
                        }
                    },
                    Some("step") => {
                        if let Err(e) = em.emulate_single() {
                            println!("Error: {:?}", e);
                        }
                    }
                    Some("info") => {
                        match em_cmd.next() {
                            Some("reg") => em.print_registers(),
                            Some("instr") => {
                                let current_instr = em.get_instr(0).unwrap();
                                let next_instr = em.get_instr(2).unwrap();
                                println!(
                                    "Current Instruction: x{:X}{:X}{:X}{:X}",
                                    current_instr.0,
                                    current_instr.1,
                                    current_instr.2,
                                    current_instr.3
                                );
                                println!(
                                    "Next Instruction: x{:X}{:X}{:X}{:X}",
                                    next_instr.0, next_instr.1, next_instr.2, next_instr.3
                                );
                            }
                            _ => println!("Unknown"),
                        };
                    }
                    _ => {
                        println!("Unknown");
                    }
                };
            }
        }
    };
}

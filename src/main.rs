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
                    Some("run") => while em.emulate_single() {},
                    Some("step") => {
                        em.emulate_single();
                    }
                    Some("info") => {
                        match em_cmd.next() {
                            Some("reg") => em.print_registers(),
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

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

            std::fs::write("./test.g3a", compiled_file.serialize("/test.g3a")).unwrap();
        }
        Rizm::Emulate { input } => {
            let raw_file = std::fs::read(input).unwrap();
            let file = g3a::File::parse(&raw_file).unwrap();

            let mut em = emulator::Emulator::new(file);
            while em.emulate_single() {}
        }
    };
}

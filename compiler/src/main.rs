use compiler;
use g3a;

use chrono::Utc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct RizmCompile {
    #[structopt(short = "i")]
    input: String,
    #[structopt(short = "o")]
    output: String,
}

fn main() {
    let cmd = RizmCompile::from_args();

    // Actually compiling a program
    let compiled_code = compiler::compile_file(cmd.input);

    let mut compiled_file_builder =
        g3a::FileBuilder::new("test".to_string(), Utc::now().naive_utc());
    compiled_file_builder
        .short_name("test".to_string())
        .internal_name("@TEST".to_string())
        .code(compiled_code);
    let compiled_file = compiled_file_builder.finish();

    let output_path = std::path::Path::new(&cmd.output);
    let output_name = output_path.file_name().unwrap();
    std::fs::write(
        output_path,
        compiled_file.serialize(&format!("/{}", output_name.to_str().unwrap())),
    )
    .unwrap();
}

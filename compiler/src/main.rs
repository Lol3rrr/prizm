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

/// The Configuration options for the Program that
/// is being compiled
struct Config {
    /// The general Name of the Application
    name: String,
    /// The Short-Name of the Application
    short_name: String,
    /// The Internal-Name of the Application as seen,
    /// by the Operating-System
    internal_name: String,
}

fn main() {
    let cmd = RizmCompile::from_args();

    let conf = Config {
        name: "test".to_string(),
        short_name: "test".to_string(),
        internal_name: "@TEST".to_string(),
    };

    // Actually compiling a program
    let compiled_code = compiler::compile_file(cmd.input);

    let mut compiled_file_builder =
        g3a::FileBuilder::new(conf.name.clone(), Utc::now().naive_utc());
    compiled_file_builder
        .short_name(conf.short_name.clone())
        .internal_name(conf.internal_name.clone())
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

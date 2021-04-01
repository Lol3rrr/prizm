use casio::{compiler, g3a};

use chrono::{NaiveDate, Utc};

fn compare(og: &[u8], new: &[u8]) {
    if og.len() != new.len() {
        println!("Different Lengths: {} != {}", og.len(), new.len());
        return;
    }

    for index in 0..og.len() {
        let og_elem = og[index];
        let new_elem = new[index];

        if og_elem != new_elem {
            println!("[{:x}] {} != {}", index, og_elem, new_elem);
        }
    }
}

fn main() {
    let g3a_path = "./examples/dino.g3a";
    let g3a_content = std::fs::read(g3a_path).unwrap();
    let g3a_file = g3a::File::parse(&g3a_content).unwrap();

    let creation_date = NaiveDate::from_ymd(2020, 04, 30).and_hms(15, 04, 0);
    let mut new_file_builder = g3a::FileBuilder::new("dino".to_string(), creation_date);
    new_file_builder
        .short_name("dino".to_string())
        .internal_name("@DINO".to_string())
        .selected_image(g3a_file.selected_image.clone())
        .unselected_image(g3a_file.unselected_image.clone())
        .code(g3a_file.executable_code.clone());
    let new_file = new_file_builder.finish();

    let og_serialized = g3a_file.serialize("/dino_game.g3a");
    let new_serialized = new_file.serialize("/dino_game.g3a");
    compare(&og_serialized, &new_serialized);

    // Actually compiling a program
    let compiler_path = "./examples/simple.c";
    let compiler_content = std::fs::read_to_string(compiler_path).unwrap();
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

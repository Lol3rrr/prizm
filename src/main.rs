use casio::g3a;

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

    let serialized = g3a_file.serialize();
    compare(&g3a_content, &serialized);

    std::fs::write("dino.g3a", &serialized).unwrap();
}

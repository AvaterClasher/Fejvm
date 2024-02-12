use std::path::PathBuf;
use Fejvm::class_file::ClassFile;
use Fejvm::class_reader;

pub fn read_class_from_file(file: &str) -> ClassFile {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/Fejvm");
    path.push(String::from(file) + ".class");
    println!("Reading class from file: {}", path.display());

    class_reader::read(path.as_path()).unwrap()
}

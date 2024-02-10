use std::path::PathBuf;

extern crate Fejvm;

#[test]
fn can_read_class_file() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/Fejvm/hi.class");
    let data = std::fs::read(path).unwrap();

    let class_file = Fejvm::class::read(&data).unwrap();
    assert_eq!("todo", class_file.name)
}

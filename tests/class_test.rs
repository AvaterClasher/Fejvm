use std::path::PathBuf;
use Fejvm::{
    class_access_flags::ClassAccessFlags, class_file_version::ClassFileVersion, class_reader,
};

extern crate Fejvm;

#[test]
fn can_read_class_file() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/Fejvm/hi.class");
    let class = class_reader::read(path.as_path()).unwrap();
    assert_eq!(ClassFileVersion::Jdk6, class.version);
    assert_eq!(
        ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        class.flags
    );
    assert_eq!("Fejvm/hi", class.name);
    assert_eq!("java/lang/Object", class.superclass);
}

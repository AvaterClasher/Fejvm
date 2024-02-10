use std::fmt;

use crate::{class_access_flags::ClassAccessFlags, c_pool::ConstantPool, class_file_version::ClassFileVersion};

/// Represents the content of a .class file.
#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constants: ConstantPool,
    pub flags: ClassAccessFlags,
    pub name: String,
    pub superclass: String,
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Class {} (extends {}), version: {}",
            self.name, self.superclass, self.version
        )?;
        write!(f, "{}", self.constants)?;
        writeln!(f, "{:?}", self.flags)
    }
}
use std::{fmt, vec::Vec};
use thiserror::Error;

// Types of constant
// Constant Pool Entry defined here
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Debug, PartialEq)]
pub enum ConstantPoolEntry {
    String(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassReference(u16),
    StringReference(u16),
    FieldReference(u16, u16),
    MethodReference(u16, u16),
    InterfaceMethodReference(u16, u16),
    NameAndTypeDescriptor(u16, u16),
}

// Constant Pool Physics Entry is Defined here
#[derive(Debug)]
enum ConstantPoolPhyEntry {
    Entry(ConstantPoolEntry),
    MultiByteEntryTombstone(),
}

// Implementation of the constant pool of a java class.
// Note that constants are 1-based in java.
#[derive(Debug, Default)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolPhyEntry>,
}

// Error used to signal that an attempt was made to access a non existing constant pool entry.
#[derive(Error, Debug, PartialEq)]
#[error("invalid constant pool index: {index}")]
pub struct InvalidConstantPoolIndexError {
    pub index: u16,
}

// Implement methods for the error type
impl InvalidConstantPoolIndexError {
    fn new(index: u16) -> Self {
        InvalidConstantPoolIndexError { index }
    }
}

// Implement methods for the constant pool struct
impl ConstantPool {
    // Constructor for creating a new constant pool
    pub fn new() -> ConstantPool {
        Default::default()
    }

    // Adds a new entry.
    pub fn add(&mut self, entry: ConstantPoolEntry) {
        // Check if the entry type requires a tombstone (e.g., Long or Double)
        let add_tombstone = matches!(
            &entry,
            ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_)
        );

        // Push the entry to the constant pool
        self.entries.push(ConstantPoolPhyEntry::Entry(entry));

        // If a tombstone is needed, add a tombstone entry
        if add_tombstone {
            self.entries
                .push(ConstantPoolPhyEntry::MultiByteEntryTombstone())
        }
    }

    // Accesses an entry given its index. Note that it must be 1-based!
    pub fn get(
        &self,
        input_index: u16,
    ) -> Result<&ConstantPoolEntry, InvalidConstantPoolIndexError> {
        // Check if the index is valid
        if input_index == 0 || input_index as usize > self.entries.len() {
            Err(InvalidConstantPoolIndexError::new(input_index))
        } else {
            // Adjust the index to be 0-based
            let i = (input_index - 1) as usize;
            let entry = &self.entries[i];

            // Return the entry if it is a valid entry, otherwise return an error
            match entry {
                ConstantPoolPhyEntry::Entry(entry) => Ok(entry),
                ConstantPoolPhyEntry::MultiByteEntryTombstone() => {
                    Err(InvalidConstantPoolIndexError::new(input_index))
                }
            }
        }
    }

    // Helper method for formatting an entry for display
    fn fmt_entry(&self, idx: u16) -> Result<String, InvalidConstantPoolIndexError> {
        let entry = self.get(idx)?;
        let text = match entry {
            // Format each type of constant pool entry
            ConstantPoolEntry::String(ref s) => format!("String: \"{}\"", s),
            ConstantPoolEntry::Integer(n) => format!("Integer: {}", n),
            ConstantPoolEntry::Float(n) => format!("Float: {}", n),
            ConstantPoolEntry::Long(n) => format!("Long: {}", n),
            ConstantPoolEntry::Double(n) => format!("Double: {}", n),
            ConstantPoolEntry::ClassReference(n) => {
                format!("ClassReference: {} => ({})", n, self.fmt_entry(*n)?)
            }
            ConstantPoolEntry::StringReference(n) => {
                format!("StringReference: {} => ({})", n, self.fmt_entry(*n)?)
            }
            ConstantPoolEntry::FieldReference(i, j) => {
                format!(
                    "FieldReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            ConstantPoolEntry::MethodReference(i, j) => {
                format!(
                    "MethodReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            ConstantPoolEntry::InterfaceMethodReference(i, j) => {
                format!(
                    "InterfaceMethodReference: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(*i)?,
                    self.fmt_entry(*j)?
                )
            }
            &ConstantPoolEntry::NameAndTypeDescriptor(i, j) => {
                format!(
                    "NameAndTypeDescriptor: {}, {} => ({}), ({})",
                    i,
                    j,
                    self.fmt_entry(i)?,
                    self.fmt_entry(j)?
                )
            }
        };
        Ok(text)
    }

    // Method for getting the textual representation of an entry
    pub fn text_of(&self, idx: u16) -> Result<String, InvalidConstantPoolIndexError> {
        let entry = self.get(idx)?;
        let text = match entry {
            // Extract text from each type of constant pool entry
            ConstantPoolEntry::String(ref s) => s.clone(),
            ConstantPoolEntry::Integer(n) => n.to_string(),
            ConstantPoolEntry::Float(n) => n.to_string(),
            ConstantPoolEntry::Long(n) => n.to_string(),
            ConstantPoolEntry::Double(n) => n.to_string(),
            ConstantPoolEntry::ClassReference(n) => self.text_of(*n)?,
            ConstantPoolEntry::StringReference(n) => self.text_of(*n)?,
            ConstantPoolEntry::FieldReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::MethodReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::InterfaceMethodReference(i, j) => {
                format!("{}.{}", self.text_of(*i)?, self.text_of(*j)?)
            }
            ConstantPoolEntry::NameAndTypeDescriptor(i, j) => {
                format!("{}: {}", self.text_of(*i)?, self.text_of(*j)?)
            }
        };
        Ok(text)
    }
}

// Implement the Display trait for custom display formatting
impl fmt::Display for ConstantPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display the size of the constant pool
        writeln!(f, "Constant pool: (size: {})", self.entries.len())?;

        // Display each entry in the constant pool
        for (raw_idx, _) in self.entries.iter().enumerate() {
            let index = (raw_idx + 1) as u16;
            writeln!(f, "    {}, {}", index, self.fmt_entry(index)?)?;
        }
        Ok(())
    }
}

// Implement conversion from InvalidConstantPoolIndexError to fmt::Error
impl From<InvalidConstantPoolIndexError> for fmt::Error {
    fn from(_: InvalidConstantPoolIndexError) -> fmt::Error {
        fmt::Error {}
    }
}

// Module for unit tests
#[cfg(test)]
mod tests {
    use crate::c_pool::{ConstantPool, ConstantPoolEntry, InvalidConstantPoolIndexError};

    // Test the constant pool
    #[test]
    fn constant_pool_works() {
        // Create a new constant pool
        let mut cp = ConstantPool::new();

        // Add some entries
        cp.add(ConstantPoolEntry::String("hey".to_string()));
        cp.add(ConstantPoolEntry::Integer(1));
        cp.add(ConstantPoolEntry::Float(2.1));
        cp.add(ConstantPoolEntry::Long(123));
        cp.add(ConstantPoolEntry::Double(3.56));
        cp.add(ConstantPoolEntry::ClassReference(1));
        cp.add(ConstantPoolEntry::StringReference(1));
        cp.add(ConstantPoolEntry::String("joe".to_string()));
        cp.add(ConstantPoolEntry::FieldReference(1, 10));
        cp.add(ConstantPoolEntry::MethodReference(1, 10));
        cp.add(ConstantPoolEntry::InterfaceMethodReference(1, 10));
        cp.add(ConstantPoolEntry::NameAndTypeDescriptor(1, 10));

        // Perform assertions to check the constant pool
        assert_eq!(
            ConstantPoolEntry::String("hey".to_string()),
            *cp.get(1).unwrap()
        );
        assert_eq!(ConstantPoolEntry::Integer(1), *cp.get(2).unwrap());
        assert_eq!(ConstantPoolEntry::Float(2.1), *cp.get(3).unwrap());
        assert_eq!(ConstantPoolEntry::Long(123i64), *cp.get(4).unwrap());
        assert_eq!(Err(InvalidConstantPoolIndexError::new(5)), cp.get(5));
        assert_eq!(ConstantPoolEntry::Double(3.56), *cp.get(6).unwrap());
        assert_eq!(Err(InvalidConstantPoolIndexError::new(7)), cp.get(7));
        assert_eq!(ConstantPoolEntry::ClassReference(1), *cp.get(8).unwrap());
        assert_eq!(ConstantPoolEntry::StringReference(1), *cp.get(9).unwrap());
        assert_eq!(
            ConstantPoolEntry::String("joe".to_string()),
            *cp.get(10).unwrap()
        );
        assert_eq!(
            ConstantPoolEntry::FieldReference(1, 10),
            *cp.get(11).unwrap()
        );
        assert_eq!(
            ConstantPoolEntry::MethodReference(1, 10),
            *cp.get(12).unwrap()
        );
        assert_eq!(
            ConstantPoolEntry::InterfaceMethodReference(1, 10),
            *cp.get(13).unwrap()
        );
        assert_eq!(
            ConstantPoolEntry::NameAndTypeDescriptor(1, 10),
            *cp.get(14).unwrap()
        );

        // Perform assertions to check the textual representation of the constant pool
        assert_eq!("hey", cp.text_of(1).unwrap());
        assert_eq!("1", cp.text_of(2).unwrap());
        assert_eq!("2.1", cp.text_of(3).unwrap());
        assert_eq!("123", cp.text_of(4).unwrap());
        assert_eq!(Err(InvalidConstantPoolIndexError::new(5)), cp.text_of(5));
        assert_eq!("3.56", cp.text_of(6).unwrap());
        assert_eq!(Err(InvalidConstantPoolIndexError::new(7)), cp.text_of(7));
        assert_eq!("hey", cp.text_of(8).unwrap());
        assert_eq!("hey", cp.text_of(9).unwrap());
        assert_eq!("joe", cp.text_of(10).unwrap());
        assert_eq!("hey.joe", cp.text_of(11).unwrap());
        assert_eq!("hey.joe", cp.text_of(12).unwrap());
        assert_eq!("hey.joe", cp.text_of(13).unwrap());
        assert_eq!("hey: joe", cp.text_of(14).unwrap());
    }
}

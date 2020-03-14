use std::{fmt, io};

use crate::{Atom, Data};

/// A structure representing the different types of content an Atom might have.
#[derive(Clone)]
pub enum Content {
    /// A value containing `Vec<Atom>`.
    Atoms(Vec<Atom>),
    /// A value containing raw `Data`.
    RawData(Data),
    /// A value containing `Data` defined by a [Table 3-5 Well-known data types](https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/Metadata/Metadata.html#//apple_ref/doc/uid/TP40000939-CH1-SW34) code.
    TypedData(Data),
    /// Empty `Content`.
    Empty,
}

impl Content {
    /// Creates a new content of type `Content::Atoms` containing an empty `Vec`.
    pub fn atoms() -> Content {
        Content::Atoms(Vec::new())
    }

    /// Creates a new content of type `Content::Atoms` containing the atom.
    pub fn atom(atom: Atom) -> Content {
        Content::Atoms(vec![atom])
    }

    /// Creates a new content of type `Content::Atoms` containing a data `Atom`.
    pub fn data_atom() -> Content { Content::atom(Atom::data_atom()) }

    /// Creates a new content of type `Content::Atoms` containing a data `Atom` with the data.
    pub fn data_atom_with(data: Data) -> Content { Content::atom(Atom::data_atom_with(data)) }

    /// Creates a new `Content` of type `Content::Atoms` containing a new `Atom` with the head,
    /// offset and content.
    pub fn atom_with(head: [u8; 4], offset: usize, content: Content) -> Content {
        Content::atom(Atom::with(head, offset, content))
    }

    /// Adds the atom to the list of children atoms if `self` is of type `Content::Atoms`.
    pub fn add_atom(self, atom: Atom) -> Content {
        if let Content::Atoms(mut atoms) = self {
            atoms.push(atom);
            Content::Atoms(atoms)
        } else {
            self
        }
    }

    /// Adds a data `Atom` to the list of children if `self` is of type `Content::Atoms`.
    pub fn add_data_atom(self) -> Content {
        self.add_atom(Atom::data_atom())
    }

    /// Adds a new `Atom` with the provided head, offset and content to the list of children if
    /// `self` is of type `Content::Atoms`.
    pub fn add_atom_with(self, head: [u8; 4], offset: usize, content: Content) -> Content {
        self.add_atom(Atom::with(head, offset, content))
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        match self {
            Content::Atoms(v) => v.iter().map(|a| a.len()).sum(),
            Content::TypedData(d) => 8 + d.len(),
            Content::RawData(d) => d.len(),
            Content::Empty => 0,
        }
    }

    /// Attempts to parse itself from the reader.
    pub fn parse(&mut self, reader: &mut (impl io::Read + io::Seek), length: usize) -> crate::Result<()> {
        match self {
            Content::Atoms(v) => Atom::parse_atoms(v, reader, length)?,
            Content::RawData(d) => d.parse(reader, length)?,
            Content::TypedData(d) => d.parse(reader, length)?,
            Content::Empty => (),
        }

        Ok(())
    }

    pub fn write_to(&self, writer: &mut impl io::Write) -> crate::Result<()> {
        match self {
            Content::Atoms(v) => for a in v {
                a.write_to(writer)?;
            }
            Content::RawData(d) => d.write_raw(writer)?,
            Content::TypedData(d) => d.write_typed(writer)?,
            Content::Empty => (),
        }

        Ok(())
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Content::Atoms(v) => if let Content::Atoms(ov) = other {
                return v == ov;
            }
            Content::RawData(d) => if let Content::RawData(od) = other {
                return d == od;
            }
            Content::TypedData(d) => if let Content::TypedData(od) = other {
                return d == od;
            }
            Content::Empty => if let Content::Empty = other { return true; }
        }

        false
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            Content::Atoms(v) => if let Content::Atoms(ov) = other {
                return v != ov;
            }
            Content::RawData(d) => if let Content::RawData(od) = other {
                return d != od;
            }
            Content::TypedData(d) => if let Content::TypedData(od) = other {
                return d != od;
            }
            Content::Empty => if let Content::Empty = other { return true; }
        }

        true
    }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Atoms(a) => write!(f, "Content::Atoms{{ {:#?} }}", a),
            Content::TypedData(d) => write!(f, "Content::TypedData{{ {:?} }}", d),
            Content::RawData(d) => write!(f, "Content::RawData{{ {:?} }}", d),
            Content::Empty => write!(f, "Content::Empty")
        }
    }
}
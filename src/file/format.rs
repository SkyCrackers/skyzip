use std::fmt::Formatter;
use std::intrinsics::transmute;
use std::iter::FromIterator;

use linked_list::LinkedList;
use serde::{Deserialize, Deserializer};
use serde::de::value::BytesDeserializer;
use serde::de::Visitor;

use crate::file::ArchiveFile;
use crate::file::error::Error;

struct ArchiveFileVisitor;

impl<'de> Visitor<'de> for ArchiveFileVisitor {
	type Value = ArchiveFile;

	fn expecting(&self, _: &mut Formatter) -> std::fmt::Result {
		todo!()
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: serde::de::Error {
		// read bytes as u32 array
		let bytes = unsafe { transmute::<&[u8], &[u32]>(v) };

		// get a linked list and cursor from bytes
		let mut list = LinkedList::from_iter(bytes.into_iter());
		let mut buf = list.cursor();

		// store some variables for later
		buf.next();
		let version = **buf.next().unwrap();
		buf.seek_forward(2);
		let align = **buf.next().unwrap();
		buf.next();
		let names_offset = **buf.next().unwrap();

		// make sure to reset the cursor head before we read the rest of the file
		buf.reset();

		// put bytes into struct
		let out = ArchiveFile {
			header: **buf.next().unwrap(),
			version: {
				buf.next();
				version
			},
			_dummy: **buf.next().unwrap(),
			files: **buf.next().unwrap(),
			align: {
				buf.next();
				align
			},
			_dummy2: **buf.next().unwrap(),
			names_offset: {
				buf.next();
				names_offset
			},
			names_offset2: {
				if version < 0xB && names_offset < align {
					Some(**buf.next().unwrap())
				} else {
					None
				}
			},
			_dummy3: **buf.next().unwrap(),
			_zero: **buf.next().unwrap(),
			_dummy4: **buf.next().unwrap(),
			names_offset3: {
				if version >= 9 {
					Some(**buf.next().unwrap())
				} else {
					None
				}
			},
			_dummy5: {
				if version >= 9 {
					Some(**buf.next().unwrap())
				} else {
					None
				}
			},
			_dummy6: {
				if version >= 9 {
					Some(**buf.next().unwrap())
				} else {
					None
				}
			},
			_dummy7: **buf.next().unwrap(),
			_zero2: {
				if version < 9 {
					Some(**buf.next().unwrap())
				} else {
					None
				}
			},
			crc: vec![]
		};
		Ok(out)
	}
}

impl<'de> From<&'de [u8]> for ArchiveFile {
	fn from(bytes: &[u8]) -> Self {
		Self::deserialize(BytesDeserializer::<Error>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for ArchiveFile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(ArchiveFileVisitor)
	}
}

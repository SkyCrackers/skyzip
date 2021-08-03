use crate::file::ArchiveFile;
use std::intrinsics::transmute;
use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

const NAME: &'static str = "ArchiveFile";
const FIELDS: &'static [&'static str] = &[
	"header",
	"version",
	"_dummy",
	"files",
	"align",
	"_dummy2",
	"names_offset",
	"names_offset2"
];

struct ArchiveFileVisitor;

impl<'de> Visitor<'de> for ArchiveFileVisitor {
	type Value = ArchiveFile;

	fn expecting(&self, _: &mut Formatter) -> std::fmt::Result {
		todo!()
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: serde::de::Error {
		// read bytes as u32 array
		let buf = unsafe { transmute::<&[u8], &[u32]>(v) };

		// put bytes into struct
		let out = ArchiveFile {
			header: buf[0],
			version: buf[1],
			_dummy: buf[2],
			files: buf[3],
			align: buf[4],
			_dummy2: buf[5],
			names_offset: buf[6],
			names_offset2: {
				let names_offset = buf[6];
				let align = buf[4];
				if names_offset < align {
					Some(buf[7])
				} else {
					None
				}
			}
		};
		Ok(out)
	}
}

impl<'de> Deserialize<'de> for ArchiveFile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_struct(NAME, FIELDS, ArchiveFileVisitor)
	}
}

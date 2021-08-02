use crate::file::ArchiveFile;
use std::intrinsics::transmute;
use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

struct ArchiveFileVisitor;

impl<'de> Visitor<'de> for ArchiveFileVisitor {
	type Value = ArchiveFile;

	fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
		todo!()
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: serde::de::Error {
		// read bytes as u32 array
		let v = unsafe { transmute::<&[u8], &[u32]>(v) };

		// put bytes into struct
		let out = ArchiveFile {
			header: v[0],
			version: v[1],
			_dummy: v[2],
			files: v[3],
			align: v[4],
			_dummy2: v[5],
			names_offset: v[6],
			names_offset2: {
				let names_offset = v[6];
				let align = v[4];
				if names_offset < align {
					Some(v[7])
				} else {
					None
				}
			}
		};
		Ok(out)
	}
}

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

impl<'de> Deserialize<'de> for ArchiveFile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_struct(NAME, FIELDS, ArchiveFileVisitor)
	}
}

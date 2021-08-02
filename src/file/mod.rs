pub mod format;
pub mod error;

use error::Error;
use std::fmt::{Formatter, Display};
use std::mem::transmute;use serde::{Deserialize, Deserializer};
use serde::de::{Visitor, StdError};
use serde::de::value::BytesDeserializer;

pub trait File<'de> where Self: Deserialize<'de> + From<&'de [u8]> {
	fn new(header: u32, version: u32, files: u32, align: u32, names_offset: (u32, Option<u32>)) -> Self;
	fn header(&self) -> u32;
	fn version(&self) -> u32;
	fn files(&self) -> u32;
	fn align(&self) -> u32;
	fn names_offset(&self) -> (u32, Option<u32>);
}

#[derive(Debug)]
pub struct ArchiveFile {
	header: u32,
	version: u32,
	_dummy: u32,
	files: u32,
	align: u32,
	_dummy2: u32,
	names_offset: u32,
	names_offset2: Option<u32>,
}

impl<'de> File<'de> for ArchiveFile {
	fn new(header: u32, version: u32, files: u32, align: u32, names_offset: (u32, Option<u32>)) -> Self {
		Self {
			header,
			version,
			_dummy: 0x10,
			files,
			align,
			_dummy2: 0xFFFFFFFF,
			names_offset: names_offset.0,
			names_offset2: names_offset.1,
		}
	}

	fn header(&self) -> u32 {
		self.header
	}

	fn version(&self) -> u32 {
		self.version
	}

	fn files(&self) -> u32 {
		self.files
	}

	fn align(&self) -> u32 {
		self.align
	}

	fn names_offset(&self) -> (u32, Option<u32>) {
		(self.names_offset, self.names_offset2)
	}
}

impl<'de> From<&'de [u8]> for ArchiveFile {
	fn from(bytes: &[u8]) -> Self {
		Self::deserialize(BytesDeserializer::<Error>::new(bytes)).unwrap()
	}
}

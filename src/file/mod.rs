use serde::Deserialize;

pub mod format;
pub mod error;

pub trait File<'de> where Self: Deserialize<'de> + From<&'de [u8]> {
	fn new() -> Self;
	fn header(&self) -> u32;
	fn version(&self) -> u32;
	fn files(&self) -> u32;
	fn align(&self) -> u32;
	fn names_offset(&self) -> (u32, Option<u32>);
	fn crc(&self) -> &Vec<u32>;
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
	
	_dummy3: u32,

	_zero: u32,

	_dummy4: u32,
	
	names_offset3: Option<u32>,
	
	_dummy5: Option<u32>,
	_dummy6: Option<u32>,
	_dummy7: u32,

	_zero2: Option<u32>,
	
	crc: Vec<u32>,
}

impl<'de> File<'de> for ArchiveFile {
	fn new() -> Self {
		Self {
			header: 0,
			version: 0,
			_dummy: 0,
			files: 0,
			align: 0,
			_dummy2: 0,
			names_offset: 0,
			names_offset2: None,
			_dummy3: 0,
			_zero: 0,
			_dummy4: 0,
			names_offset3: None,
			_dummy5: None,
			_dummy6: None,
			_dummy7: 0,
			_zero2: None,
			crc: vec![],
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

	fn crc(&self) -> &Vec<u32> {
		&self.crc
	}
}

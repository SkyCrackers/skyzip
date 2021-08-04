use crate::file::{ArchiveFile, File};

#[test]
fn test() {
	// read file to byte array
	let bytes: &[u8] = include_bytes!("../run/cardgame_card_000.arc");

	// read file to struct
	let file = ArchiveFile::from(bytes);

	println!("{:#x?}", file);

	let _header_bytes = file.header().to_ne_bytes();
	let header = String::from_utf8_lossy(&_header_bytes);

	println!("{}", header);
}

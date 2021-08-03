use crate::file::{ArchiveFile, File};

#[test]
fn test() {
	// cast from slice to array
	let bytes: &[u8] = include_bytes!("../run/cardgame_card_000.arc");
	// read file
	let file = ArchiveFile::from(bytes);
	// print file
	println!("{:#x?}", file);

	let _h = file.header().to_ne_bytes();
	// header string
	let header = String::from_utf8_lossy(&_h);

	// print header
	println!("{}", header);
}

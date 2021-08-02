use crate::file::ArchiveFile;

#[test]
fn test() {
	// cast from slice to array
	let bytes: &[u8] = include_bytes!("../run/cardgame_card_000.arc");
	// read file
	let file = ArchiveFile::from(bytes);
	// print file
	println!("{:#x?}", file);
}

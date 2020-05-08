use assert_cmd::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn basic_use() -> Result<(), Box<std::error::Error>> {
	let dir = tempdir()?;

	let file_a_path = dir.path().join("a.txt");
	let file_a = fs::File::create(file_a_path)?;
	let file_c_path = dir.path().join("c.md");
	let file_c = fs::File::create(file_c_path)?;
	let dir_b_path = dir.path().join("b");
	fs::create_dir(dir_b_path)?;
	let file_b_path = dir.path().join("b").join("b.txt");
	let file_b = fs::File::create(file_b_path);

	let mut cmd = Command::cargo_bin("relbl")?;
	cmd.arg(r"(.*)\.txt")
		.arg("${1}-text.txt")
		.arg("--target-dir")
		.arg(dir.path());
	cmd.assert().success();

	assert!(file_exists(dir.path().join("a-text.txt")));
	assert!(!file_exists(dir.path().join("a.txt")));
	assert!(file_exists(dir.path().join("c.md")));
	assert!(file_exists(dir.path().join("b").join("b.txt")));
	drop(file_a);
	drop(file_b);
	drop(file_c);
	dir.close()?;
	Ok(())
}

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
	fs::File::open(path).is_ok()
}

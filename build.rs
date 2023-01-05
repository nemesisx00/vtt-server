#![allow(non_snake_case, non_upper_case_globals)]

use std::{
	env,
	fs::{
		create_dir_all,
		read_dir,
		remove_dir_all,
		DirEntry,
		OpenOptions,
	},
	io::copy,
	path::Path,
	process::Command,
};

const destinationDirectory: &str = "dist";

/// Generate the WASM output from vtt-server-web
fn main()
{
	let webDir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/web"));
	
	let mut trunkProcess = Command::new("trunk")
		.args(["build"])
		.current_dir(&webDir)
		.spawn()
		.expect("Trunk failed to build vtt-server-web!");
	
	trunkProcess.wait()
		.expect("Panicked waiting for trunkProcess to end!");
	
	let fromDir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/target/dist"));
	// Move vtt-server-web's dist/ contents next to the vtt-server output for easier running
	let toPathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("/target")
		.join(env::var("PROFILE").unwrap().as_str())
		.join(destinationDirectory);
	let toPath = toPathBuf.as_path();
	
	if toPath.exists()
	{
		remove_dir_all(toPath)
			.expect("Failed to clean up the destination directory!");
	}
	
	copyDir(&fromDir, toPath);
	
	//Re-run every time
	println!("cargo:rerun-if-changed=target");
}

fn copyDir(from: &Path, to: &Path)
{
	create_dir_all(to).unwrap();
	for entry in read_dir(from).unwrap()
	{
		let entryData = entry.unwrap();
		match entryData.path().is_dir()
		{
			true => copyDir(&entryData.path(), to.join(entryData.file_name()).as_path()),
			false => copyFile(entryData, to),
		}
	}
}

fn copyFile(entry: DirEntry, to: &Path)
{
	let mut reader = OpenOptions::new()
		.read(true)
		.open(entry.path())
		.unwrap();
	let mut writer = OpenOptions::new()
		.create(true)
		.write(true)
		.open(to.join(entry.file_name()))
		.unwrap();
	copy(&mut reader, &mut writer)
		.expect("Failed to copy file!");
}

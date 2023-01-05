#![allow(non_snake_case)]

use std::{
	path::Path,
	process::Command,
};

/// Generate the WASM output from vtt-server-web
fn main()
{
	let webDir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/web"));
	
	Command::new("trunk")
		.args(["build"])
		.current_dir(&webDir)
		.spawn()
		.expect("Trunk failed to build vtt-server-web!");
	
	//Re-run every time
	println!("cargo:rerun-if-changed=target");
}

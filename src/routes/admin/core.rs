#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::{
	Path,
	PathBuf,
};
use actix_files::NamedFile;
use actix_web::{
	get,
	HttpRequest,
	Result,
};

#[get("/")]
pub async fn home() -> Result<NamedFile>
{
	//TODO: if validate(json)
	let path = Path::new("./admin/index.html");
	return Ok(NamedFile::open(path)?);
}

#[get("/{filename:.*(.css|.js|.wasm)}")]
pub async fn web(req: HttpRequest) -> Result<NamedFile>
{
	let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
	let path = Path::new("./admin").join(filename.to_owned());
	return Ok(NamedFile::open(path)?);
}

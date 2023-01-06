#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::path::Path;
use actix_files::NamedFile;
use actix_web::{
	get,
	post,
	HttpResponse,
	HttpRequest,
	Responder,
	Result,
};

#[get("/")]
pub async fn home() -> Result<NamedFile>
{
	let path = Path::new("./admin/index.html");
	return Ok(NamedFile::open(path).unwrap());
}

#[get("/{filename}.{extension}")]
pub async fn getFile(req: HttpRequest) -> Result<NamedFile>
{
	let filename = req.match_info().get("filename").unwrap();
	let extension = req.match_info().get("extension").unwrap();
	let path = Path::new("./admin").join(filename.to_owned() + "." + extension);
	return Ok(NamedFile::open(path).unwrap());
}

#[get("/snippets/{dioxus}/src/interpreter.js")]
pub async fn getInterpreter(req: HttpRequest) -> Result<NamedFile>
{
	let folder = req.match_info().get("dioxus").unwrap();
	let path = Path::new("./admin/snippets/").join(folder).join("src/interpreter.js");
	return Ok(NamedFile::open(path).unwrap());
}

#[post("/user/new")]
pub async fn userNew(json: String) -> impl Responder
{
	return HttpResponse::Ok().body(json);
}

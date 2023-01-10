#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::database::{
	allUsers,
};
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
	//TODO: if validate(json)
	let path = Path::new("./admin/index.html");
	let file = NamedFile::open(path)?;
	return Ok(file);
}

#[get("/{filename}.{extension}")]
pub async fn getFile(req: HttpRequest) -> Result<NamedFile>
{
	let filename = req.match_info().get("filename").unwrap();
	let extension = req.match_info().get("extension").unwrap();
	let path = Path::new("./admin").join(filename.to_owned() + "." + extension);
	let file = NamedFile::open(path)?;
	return Ok(file);
}

#[get("/snippets/{dioxus}/src/interpreter.js")]
pub async fn getInterpreter(req: HttpRequest) -> Result<NamedFile>
{
	let folder = req.match_info().get("dioxus").unwrap();
	let path = Path::new("./admin/snippets/").join(folder).join("src/interpreter.js");
	let file = NamedFile::open(path)?;
	return Ok(file);
}

#[post("/user/new")]
pub async fn userNew(json: String) -> impl Responder
{
	//TODO: if validate(json)
	return HttpResponse::Ok().body(json);
}

#[post("/user/list")]
pub async fn userList(args: String) -> impl Responder
{
	println!("POST /admin/user/list: {}", args);
	//TODO: if validate(json)
	let users = allUsers().await;
	let json = serde_json::to_string(&users).unwrap();
	return HttpResponse::Ok().body(json);
}

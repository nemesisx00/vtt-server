#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use actix_web::{
	get,
	post,
	App,
	HttpResponse,
	HttpServer,
	Responder,
};

#[get("/")]
async fn hello() -> impl Responder
{
	HttpResponse::Ok().body("Hello VTT World!")
}

#[post("/echo")]
async fn echo(reqBody: String) -> impl Responder
{
	HttpResponse::Ok().body(reqBody)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	HttpServer::new(||
	{
		App::new()
			.service(hello)
			.service(echo)
	})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use actix_web::{
	dev::{
		forward_ready,
		Service,
		ServiceRequest,
		ServiceResponse,
		Transform,
	},
	Error,
};
use futures::future::{
	ready,
	Ready,
	LocalBoxFuture,
};
use log::info;

pub struct AuthenticationService;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationService
	where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = AuthenticationMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;
	
	fn new_transform(&self, service: S) -> Self::Future
	{
		return ready(Ok(AuthenticationMiddleware { service }));
	}
}

#[derive(Clone)]
pub struct AuthenticationMiddleware<S>
{
	service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
	where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
	
	forward_ready!(service);
	
	fn call(&self, req: ServiceRequest) -> Self::Future
	{
		info!("Authentication Middleware processing request: {}", req.path());
		
		let fut = self.service.call(req);
		
		return Box::pin(async move {
			let res = fut.await?;
			
			info!("Authentication Middleware Response");
			return Ok(res);
		});
	}
}

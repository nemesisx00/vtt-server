#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::structs::{
	User,
};
use std::{
	collections::HashMap,
	error::Error,
};
use dioxus::prelude::*;
use dioxus::core::to_owned;
use reqwest::{
	Client,
};

async fn getUsers() -> Result<Vec<User>, Box<dyn Error>>
{
	let resp = Client::new()
		.post("http://127.0.0.1:8080/admin/user/list")
		.form(&[
			("token", "some unique token"),
		])
		.send()
		.await?
		.json::<Vec<User>>()
		.await;
	
	let mut o = vec![];
	match resp
	{
		Ok(res) => o = res.to_owned(),
		Err(e) => log::error!("Failed to retrieve Users list: {:?}", e),
	}
	
	return Ok(o);
}

pub fn ManageUsers(cx: Scope) -> Element
{
	let usernames = use_ref(&cx, || HashMap::<i64, String>::new());
	
	let refreshHandler = move |_| {
		to_owned![usernames];
		cx.spawn(async move {
			match getUsers().await
			{
				Ok(fetched) => {
					usernames.write().clear();
					for user in fetched
					{
						usernames.write().insert(user.id, user.label);
					}
				},
				_ => log::error!("getUsers() failed!"),
			};
		})
	};
	
	return cx.render(rsx!{
		div
		{
			"Manage Users!"
			
			button
			{
				onclick: refreshHandler,
				"Refresh User List"
			}
			
			usernames.read().iter().map(|(i, username)| rsx!(div
			{
				key: "{i}",
				class: "user",
				"{username}"
			}))
		}
	});
}
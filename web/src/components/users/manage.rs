#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	structs::User,
	util::location,
};
use std::{
	collections::HashMap,
	error::Error,
};
use dioxus::prelude::*;
use dioxus::core::to_owned;
use reqwest::{
	Client,
	Url,
};
#[allow(unused_imports)]
use log::{
	error,
	info,
};

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
			class: "manageUsers column",
			
			div
			{
				class: "heading column",
				
				"Manage Users!"
				button
				{
					onclick: refreshHandler,
					"Refresh User List"
				}
			}
			
			div
			{
				class: "userList column",
				
				usernames.read().iter().map(|(i, username)| rsx!(div
				{
					key: "{i}",
					class: "user row",
					
					div { "{username}" }
					div { class: "deleteUser", onclick: move |_| info!("Do delete dialog pop up here"), "Delete User" }
				}))
			}
		}
	});
}

async fn getUsers() -> Result<Vec<User>, Box<dyn Error>>
{
	let endpoint = Url::parse(location().as_ref())?
		.join("/user/list")?
		.to_string();
	
	let resp = Client::new()
		.post(endpoint)
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

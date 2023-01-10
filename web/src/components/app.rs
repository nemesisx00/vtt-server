#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::{
	collections::HashMap,
	error::Error,
};
use dioxus::prelude::*;
use dioxus::core::to_owned;
use reqwest::{
	Client,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct User
{
	pub id: i64,
	pub label: String,
	pub name: Option<String>,
	pub avatar: Option<String>,
	pub description: Option<String>,
}

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

pub fn App(cx: Scope) -> Element
{
	let showManageUsers = use_state(&cx, || false);
	let usernames = use_ref(&cx, || HashMap::<i64, String>::new());
	
	let manageUsersClickHandler = move |_| {
		if !showManageUsers
		{
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
			});
		}
		
		showManageUsers.set(!showManageUsers);
	};
	
	return cx.render(rsx!{
		h1
		{
			"vtt-server Administrator Interface"
		}
		hr {}
		button
		{
			onclick: manageUsersClickHandler,
			"Manage Users"
		}
		
		showManageUsers.then(|| rsx!{
			div
			{
				"Manage Users!",
				
				usernames.read().iter().map(|(i, username)| rsx!(div
				{
					key: "{i}",
					class: "user",
					"{username}"
				}))
			}
		})
	});
}

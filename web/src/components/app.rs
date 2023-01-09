#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::{
	collections::HashMap,
	error::Error,
};
use dioxus::prelude::*;
use reqwest::{
	Client,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct Users
{
	pub users: Vec<User>,
}

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
			("username", "username"),
			("password", "password"),
		])
		.send()
		.await?
		.json::<Users>()
		.await?;
	
	return Ok(resp.users);
}

pub fn App(cx: Scope) -> Element
{
	let showManageUsers = use_state(&cx, || false);
	
	let usersFuture = use_future(&cx, (), |_| async move
	{
		getUsers()
			.await
	});
	
	let mut usernames = HashMap::<i64, String>::default();
	if *showManageUsers.get()
	{
		if let Ok(users) = usersFuture.value()?
		{
			for user in users.iter()
			{
				usernames.insert(user.id, user.label.to_owned());
			}
		}
	}
	let noUsers = usernames.len() == 0;
	
	return cx.render(rsx!{
		h1
		{
			"vtt-server Administrator Interface"
		}
		hr {}
		button
		{
			onclick: move |_| {
				showManageUsers.set(!showManageUsers);
				if *showManageUsers.get()
				{
					usersFuture.restart();
				}
			},
			"Manage Users"
		}
		
		showManageUsers.then(|| rsx!{
			div
			{
				"Manage Users!",
				
				usernames.iter().map(|(i, username)| rsx!(div
				{
					key: "{i}",
					"{username}"
				})),
				
				noUsers.then(|| rsx!(div
				{
					"No users found!"
				})),
			}
		})
	});
}

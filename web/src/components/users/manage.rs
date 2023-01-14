#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	components::users::Modify,
	structs::User,
	util::endpoint,
};
use std::{
	collections::HashMap,
	error::Error,
};
use dioxus::prelude::*;
use dioxus::core::to_owned;
use reqwest::Client;
#[allow(unused_imports)]
use log::{
	error,
	info,
};

pub fn ManageUsers(cx: Scope) -> Element
{
	let users = use_ref(&cx, || HashMap::<usize, (i64, String)>::new());
	
	let refreshHandler = move |_| {
		to_owned![users];
		cx.spawn(async move {
			match getUsers().await
			{
				Ok(fetched) => {
					users.write().clear();
					let mut i = 0;
					for user in fetched
					{
						users.write().insert(i, (user.id, user.label));
						i += 1;
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
			
			Modify { create: true }
			
			div
			{
				class: "userList column",
				
				button
				{
					id: "refreshUserList",
					onclick: refreshHandler,
					"Refresh User List"
				}
				
				users.read().iter().map(|(i, (userId, label))|
				{
					let myId = userId.clone();
					
					rsx!(
						div
						{
							key: "user-{userId}",
							class: "user row",
							
							div { class: "userLabel", "{label}" }
							div
							{
								class: "deleteUser",
								title: "Delete User",
								onclick: move |_| info!("Do delete dialog pop up here for userId {}", myId)
							}
						}
						
						(*i + 1 < users.read().len()).then(|| rsx!(hr { key: "hr-{userId}" }))
					)
				})
			}
		}
	});
}

async fn getUsers() -> Result<Vec<User>, Box<dyn Error>>
{
	let resp = Client::new()
		.post(endpoint("/user/list")?)
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

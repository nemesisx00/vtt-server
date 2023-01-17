#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	components::{
		dialog::Confirm,
		users::Modify,
	},
	structs::{
		User,
		ResponseData,
	},
	util::endpoint,
};
use std::{
	collections::BTreeMap,
	error::Error,
};
use dioxus::prelude::*;
use dioxus::events::FormEvent;
use dioxus_elements::geometry::PagePoint;
use reqwest::Client;
#[allow(unused_imports)]
use log::{
	error,
	info,
};

pub fn ManageUsers(cx: Scope) -> Element
{
	let createMode = use_state(&cx, || true);
	let idToDelete = use_state(&cx, || None);
	let popupCoords = use_state(&cx, || PagePoint::default());
	let selectedUser = use_ref(&cx, || None);
	let users = use_ref(&cx, || BTreeMap::<i64, User>::new());
	
	let deleteHandler = move |_| {
		to_owned![idToDelete, users];
		
		if let Some(userId) = idToDelete.get().clone()
		{
			cx.spawn(async move {
				match deleteUser(userId).await
				{
					Ok(result) => {
						if result == true
						{
							idToDelete.set(None);
							
							match getUsers().await
							{
								Ok(fetched) => {
									users.write().clear();
									for user in fetched
									{
										users.write().insert(user.id, user.to_owned());
									}
								},
								_ => error!("getUsers() failed!"),
							};
						}
					},
					_ => error!("deleteUser({}) failed!", userId),
				};
			})
		}
	};
	
	let refreshHandler = move |_| {
		to_owned![users];
		
		cx.spawn(async move {
			match getUsers().await
			{
				Ok(fetched) => {
					users.write().clear();
					for user in fetched
					{
						users.write().insert(user.id, user.to_owned());
					}
				},
				_ => error!("getUsers() failed!"),
			};
		})
	};
	
	let submitHandler = move |e: FormEvent| {
		to_owned![users];
		
		match createMode.get()
		{
			true => cx.spawn(async move {
				let username = e.values["username"].to_owned();
				let label = e.values["label"].to_owned();
				
				match createUser(username, label).await
				{
					Ok(response) => info!("User created: {:?}", response.payload),
					Err(e) => error!("Error while creating user: {:?}", e),
				}
				
				match getUsers().await
				{
					Ok(fetched) => {
						users.write().clear();
						for user in fetched
						{
							users.write().insert(user.id, user.to_owned());
						}
					},
					_ => error!("getUsers() failed!"),
				};
			}),
			
			false => cx.spawn(async move {
				let userId = e.values["userId"].to_owned();
				let label = e.values["label"].to_owned();
				let avatar = e.values["avatar"].to_owned();
				let description = e.values["description"].to_owned();
				
				match updateUser(userId, label, avatar, description).await
				{
					Ok(response) => info!("User updated: {:?}", response.payload),
					Err(e) => error!("Error while updating user: {:?}", e),
				}
				
				match getUsers().await
				{
					Ok(fetched) => {
						users.write().clear();
						for user in fetched
						{
							users.write().insert(user.id, user.to_owned());
						}
					},
					_ => error!("getUsers() failed!"),
				};
			}),
		};
		
		createMode.set(true);
		selectedUser.set(None);
	};
	
	let showConfirm = match idToDelete.get()
	{
		Some(_) => true,
		None => false,
	};
	
	return cx.render(rsx!{
		div
		{
			class: "manageUsers column",
			
			h1 { "Manage Users" }
			
			Modify
			{
				create: createMode.get().to_owned(),
				user: selectedUser.read().to_owned(),
				onSubmit: submitHandler,
				onCancel: move |_| {
					createMode.set(true);
					selectedUser.set(None);
				}
			}
			
			button
			{
				class: "button border",
				id: "refreshUserList",
				onclick: refreshHandler,
				"Refresh User List"
			}
			
			div
			{
				class: "userList column",
				
				users.read().iter().map(|(userId, user)|
				{
					let currId = userId.to_owned();
					let currUser = user.to_owned();
					
					rsx!(
						div
						{
							key: "div{userId}",
							class: "user row",
							
							div
							{
								class: "userLabel",
								onclick: move |_| {
									createMode.set(false);
									selectedUser.set(Some(currUser.to_owned()));
								},
								"{user.label}",
							}
							
							button
							{
								class: "deleteUser button",
								title: "Delete User",
								onclick: move |e| {
									idToDelete.set(Some(currId));
									popupCoords.set(e.data.page_coordinates());
								},
							}
						}
						
						hr { key: "hr{userId}" }
					)
				})
			}
			
			showConfirm.then(|| rsx!(
				Confirm
				{
					bodyText: "Are you sure you want to delete this user?",
					acceptText: "Yes",
					cancelText: "No",
					coords: popupCoords.get().clone(),
					onAccept: deleteHandler,
					onCancel: move |_| idToDelete.set(None),
				}
			))
		}
	});
}

async fn deleteUser(id: i64) -> Result<bool, Box<dyn Error>>
{
	let resp = Client::new()
		.post(endpoint("/user/delete")?)
		.form(&[
			("userId", id),
		])
		.send()
		.await?
		.json::<ResponseData<bool>>()
		.await;
	
	return match resp
	{
		Ok(res) => match res.payload
		{
			Some(payload) => {
				info!("{}", res.message);
				Ok(payload)
			},
			None => {
				error!("{}", res.message);
				Ok(false)
			},
		},
		Err(e) => {
			error!("Failed to retrieve Users list: {:?}", e);
			Ok(false)
		},
	};
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
		Err(e) => error!("Failed to retrieve Users list: {:?}", e),
	}
	
	return Ok(o);
}

async fn createUser(username: String, label: String) -> Result<ResponseData<User>, Box<dyn Error>>
{
	let resp = Client::new()
		.post(endpoint("/user/new")?)
		.form(&[
			("username", username),
			("label", label),
		])
		.send()
		.await?
		.json::<ResponseData<User>>()
		.await;
	
	return Ok(resp?);
}

async fn updateUser(userId: String, label: String, avatar: String, description: String) -> Result<ResponseData<User>, Box<dyn Error>>
{
	let resp = Client::new()
		.post(endpoint("/user/update")?)
		.form(&[
			("userId", userId),
			("label", label),
			("avatar", avatar),
			("description", description),
		])
		.send()
		.await?
		.json::<ResponseData<User>>()
		.await;
	
	return Ok(resp?);
}

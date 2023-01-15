#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	structs::{
		User,
		ResponseData,
	},
	util::endpoint,
};
use std::error::Error;
use dioxus::prelude::*;
use dioxus::{
	events::FormEvent,
};
use log::{
	error,
	info,
};
use reqwest::Client;

#[derive(Debug, Default, PartialEq, Props)]
pub struct ModifyUserProps
{
	create: bool,
}

pub fn ModifyUser(cx: Scope<ModifyUserProps>) -> Element
{
	let userId = use_state(&cx, || "".to_owned());
	let username = use_state(&cx, || "".to_owned());
	let label = use_state(&cx, || "".to_owned());
	let avatar = use_state(&cx, || "".to_owned());
	let description = use_state(&cx, || "".to_owned());
	
	let isCreate = cx.props.create;
	let submitText = match isCreate
	{
		true => "Create",
		false => "Update",
	};
	
	return cx.render(rsx!{
		div
		{
			class: "modifyUser column",
			form
			{
				prevent_default: "onsubmit",
				onsubmit: move |e| {
					match isCreate
					{
						true => createUser(&cx, e),
						false => modifyUser(&cx, e),
					}
					
					userId.set("".to_owned());
					username.set("".to_owned());
					label.set("".to_owned());
					avatar.set("".to_owned());
					description.set("".to_owned());
				},
				
				(!isCreate).then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "userId", "User ID: " }
						input { r#type: "text", name: "userId", value: "{userId}", oninput: move |e| userId.set(e.value.to_owned()) }
					}
				))
				
				isCreate.then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "username", "Username: " }
						input { r#type: "text", name: "username", value: "{username}", oninput: move |e| username.set(e.value.to_owned()) }
					}
				))
				
				div
				{
					class: "row",
						
					label { r#for: "label", "Label: " }
					input { r#type: "text", name: "label", value: "{label}", oninput: move |e| label.set(e.value.to_owned()) }
				}
				
				(!isCreate).then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "avatar", "Avatar: " }
						input { r#type: "text", name: "avatar", value: "{avatar}", oninput: move |e| avatar.set(e.value.to_owned()) }
					}
					div
					{
						class: "row",
						
						label { r#for: "description", "Description: " }
						input { r#type: "text", name: "description", value: "{description}", oninput: move |e| description.set(e.value.to_owned()) }
					}
				))
				
				div
				{
					class: "row",
					button { class: "button border", r#type: "submit", value: "{submitText}", "{submitText} the User" }
				}
			}
		}
	});
}

fn createUser(cx: &Scope<ModifyUserProps>, e: FormEvent)
{
	println!("Submitted ModifyUser form! {:?}", e.values);
	
	let username = e.values["username"].to_owned();
	let label = e.values["label"].to_owned();
	
	cx.spawn(async move {
		match userNew(username, label).await
		{
			Ok(response) => info!("User created: {:?}", response.payload),
			Err(e) => error!("Error while creating user: {:?}", e),
		}
	});
}

fn modifyUser(cx: &Scope<ModifyUserProps>, e: FormEvent)
{
	println!("Submitted ModifyUser form! {:?}", e.values);
	
	let userId = e.values["userId"].to_owned();
	let label = e.values["label"].to_owned();
	let avatar = e.values["avatar"].to_owned();
	let description = e.values["description"].to_owned();
	
	cx.spawn(async move {
		match userUpdate(userId, label, avatar, description).await
		{
			Ok(response) => info!("User updated: {:?}", response.payload),
			Err(e) => error!("Error while updating user: {:?}", e),
		}
	});
}

async fn userNew(username: String, label: String) -> Result<ResponseData<User>, Box<dyn Error>>
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

async fn userUpdate(userId: String, label: String, avatar: String, description: String) -> Result<ResponseData<User>, Box<dyn Error>>
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

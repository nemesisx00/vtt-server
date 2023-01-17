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
	events::{
		FormEvent,
	},
};
use log::{
	error,
	info,
};
use reqwest::Client;

#[derive(Props)]
pub struct ModifyUserProps<'a>
{
	create: bool,
	
	#[props(!optional)]
	user: Option<User>,
	
	onSubmit: EventHandler<'a, FormEvent>
}

pub fn ModifyUser<'a>(cx: Scope<'a, ModifyUserProps>) -> Element<'a>
{
	info!("ModifyUser user: {:?}", &cx.props.user);
	
	let isCreate = cx.props.create;
	let submitText = match isCreate
	{
		true => "Create",
		false => "Update",
	};
	
	let user = match cx.props.user.to_owned()
	{
		Some(user) => user,
		None => User::default(),
	};
	
	let avatar = match user.avatar
	{
		Some(value) => value,
		None => "".to_string(),
	};
	
	let description = match user.description
	{
		Some(value) => value,
		None => "".to_string(),
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
						true => createUser(&cx, e.to_owned()),
						false => modifyUser(&cx, e.to_owned()),
					}
					
					cx.props.onSubmit.call(e);
				},
				
				(!isCreate).then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "userId", "User ID: " }
						input { disabled: "", r#type: "text", value: "{user.id}" }
						input { r#type: "hidden", name: "userId", value: "{user.id}" }
					}
				))
				
				isCreate.then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "username", "Username: " }
						input { r#type: "text", name: "username", value: "{user.username}" }
					}
				))
				
				div
				{
					class: "row",
						
					label { r#for: "label", "Label: " }
					input { r#type: "text", name: "label", value: "{user.label}" }
				}
				
				(!isCreate).then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "avatar", "Avatar: " }
						input { r#type: "text", name: "avatar", value: "{avatar}" }
					}
					div
					{
						class: "row",
						
						label { r#for: "description", "Description: " }
						input { r#type: "text", name: "description", value: "{description}" }
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

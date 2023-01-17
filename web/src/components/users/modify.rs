#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::structs::User;
use dioxus::prelude::*;
use dioxus::{
	events::{
		FormEvent,
	},
};

#[derive(Props)]
pub struct ModifyUserProps<'a>
{
	create: bool,
	
	#[props(!optional)]
	user: Option<User>,
	
	onSubmit: EventHandler<'a, FormEvent>,
	onCancel: EventHandler<'a, MouseEvent>,
}

pub fn ModifyUser<'a>(cx: Scope<'a, ModifyUserProps>) -> Element<'a>
{
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
				onsubmit: move |e| cx.props.onSubmit.call(e),
				
				(!isCreate).then(|| rsx!(
					div
					{
						class: "row",
						
						label { r#for: "userId", "User ID: " }
						span { "{user.id}" }
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
					class: "row formButtons",
					input { class: "button border", r#type: "submit", value: "{submitText}", "{submitText} the User" }
					(!isCreate).then(|| rsx!(
						button { class: "button border", onclick: move |e| cx.props.onCancel.call(e), "Cancel" }
					))
				}
			}
		}
	});
}

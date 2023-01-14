#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	components::{
		users::Manage,
	},
};
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element
{
	let showManageUsers = use_state(&cx, || false);
	
	return cx.render(rsx!{
		h1 { "vtt-server Administrator Interface" }
		hr {}
		button
		{
			onclick: move |_| showManageUsers.set(!showManageUsers),
			"Manage Users"
		}
		
		showManageUsers.then(|| rsx!{
			Manage {}
		})
	});
}

#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element
{
	let showManageUsers = use_state(&cx, || false);
	
	return cx.render(rsx!{
		h1
		{
			"vtt-server Administrator Interface"
		}
		hr {}
		button
		{
			onclick: move |_| showManageUsers.set(!showManageUsers),
			"Manage Users"
		}
		showManageUsers.then(|| rsx!{
			div
			{
				"Manage Users!"
			}
		})
	});
}

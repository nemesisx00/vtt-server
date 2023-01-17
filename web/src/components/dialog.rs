#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use dioxus::events::MouseEvent;
use dioxus_elements::geometry::PagePoint;

#[derive(Props)]
pub struct DialogProps<'a>
{
	#[props(default = "Ok")]
	acceptText: &'a str,
	bodyText: &'a str,
	coords: PagePoint,
	onAccept: EventHandler<'a, MouseEvent>,
}

pub fn Dialog<'a>(cx: Scope<'a, DialogProps<'a>>) -> Element<'a>
{
	let css = format!("top: {}px; left: {}px;", cx.props.coords.y - 50., cx.props.coords.x - 125.);
	
	return cx.render(rsx!{
		div
		{
			class: "dialog",
			style: "{css}",
			
			div
			{
				class: "body",
				"{cx.props.bodyText}"
			}
			
			button
			{
				class: "button border",
				onclick: move |e| cx.props.onAccept.call(e),
				"{cx.props.acceptText}"
			}
		}
	});
}

// --------------------------------------------------

#[derive(Props)]
pub struct ConfirmProps<'a>
{
	#[props(default = "Ok")]
	acceptText: &'a str,
	bodyText: &'a str,
	#[props(default = "Cancel")]
	cancelText: &'a str,
	coords: PagePoint,
	onAccept: EventHandler<'a, MouseEvent>,
	onCancel: Option<EventHandler<'a, MouseEvent>>,
}

pub fn Confirm<'a>(cx: Scope<'a, ConfirmProps<'a>>) -> Element<'a>
{
	let css = format!("top: {}px; left: {}px;", cx.props.coords.y - 50., cx.props.coords.x - 125.);
	
	return cx.render(rsx!{
		div
		{
			class: "confirm dialog",
			style: "{css}",
			
			div
			{
				class: "body",
				"{cx.props.bodyText}"
			}
			
			div
			{
				class: "row",
				
				button
				{
					class: "button border",
					onclick: move |e| cx.props.onAccept.call(e),
					"{cx.props.acceptText}"
				}
				
				button
				{
					class: "button border",
					onclick: move |e| if let Some(handler) = &cx.props.onCancel { handler.call(e); },
					"{cx.props.cancelText}"
				}
			}
		}
	});
}

use super::Params;
use crate::core::html::nodes::Node;


pub type BoxedNode<MDL, MSG> = Box<(dyn Fn(MDL, Params) -> Node<MSG>)>;

#[macro_export]
macro_rules! router_handler {
    ($model:ty, $msg:ty, $closure:expr) => {{
		#[allow(unused_mut)]
		let mut closure = $closure;
		let b: $crate::duid_routers::BoxedNode::<$model, $msg> = Box::new(move |new_model: $model, params| {
			closure(new_model, params)
		});
		b
    }};       
}


#[macro_export]
macro_rules! router {
	($model_type:ty, $msg_type:ty, $model:expr, $request_path:expr, {
        $($pattern:literal => {
            $($result:expr),*
        }),*
    }) => {{
        let res = $crate::duid_routers::match_route!(
            $request_path,
            {
				$($pattern => {
					$($result),*
				}),*
			}
        );
		let model = $model;
		match res {
			Ok((handler, params)) => {
				Ok(handler(model, params))
			},
			Err(e) => Err(e)
		}
    }};
}

#[cfg(test)]
mod router_test {
	use crate::core::html::{nodes::Node, div, text};
	use crate::routers::{Params, Error};
	use crate::router;

	#[derive(Clone, Debug)]
	struct Model {
		value: i8
	}

	#[derive(Clone, Debug)]
	enum Message {
		Msg
	}
	
	// Example views...
	fn foo_bar(_model: Model, _params: Params) -> Node<Message> {
		div(&[], &[text("Djedou")])
	}
	
	fn user_profile(_model: Model, params: Params) -> Node<Message> {
		// Extracting a parameter from the path.
		let name = params.get("name").unwrap();
		div(&[], &[text(&format!("Profile for {}", name))])
	}

	#[test]
	fn foo_path_test() {
		let res = router!(Model, Message, Model{value: 12}, "/foo/bar", {
			"/foo/bar" => {
				crate::router_handler!(Model, Message, foo_bar)
			},
			"/user/:name" => {
				crate::router_handler!(Model, Message, user_profile)
			}
		});

		println!("{:#?}", res);
	}

	#[test]
	fn user_path_test() {
		let res = router!(Model, Message, Model{value: 12}, "/user/Djedou", {
			"/foo/bar" => {
				crate::router_handler!(Model, Message, foo_bar)
			},
			"/user/:name" => {
				crate::router_handler!(Model, Message, user_profile)
			}
		});

		println!("{:#?}", res);
	}
}
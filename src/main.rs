use yew::prelude::*;

#[function_component]
fn App() -> Html {
	let counters = use_state(|| 0);
	let onclick = {
		let counters = counters.clone();
		move |_| {
			let value = *counters + 1;
			counters.set(value);
		}
	};

	html! {
		<div>
			<button {onclick}> {"+1"} </button>
			<p> {*counters} </p>
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}

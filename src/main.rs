// mod oldapp;

// use oldapp::App;

mod context;
use context::MessageProvider;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <MessageProvider/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

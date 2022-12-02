#![feature(trait_alias)]

mod editor;
mod selector;
pub mod components;
mod showcase;

use editor::TemplateEditor;
use selector::TemplateSelector;
use showcase::Showcase;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/showcase")]
    Showcase,

    #[at("/template/:pane_count")]
    TemplateEditor { pane_count: u32 },

    #[at("/")]
    TemplateSelector,

    #[not_found]
    #[at("/404")]
    NotFound,

}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            <img src="/public/polygons.jpg?v=0" class="invisible"/>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Showcase => html! { <Showcase/> },
        Route::TemplateSelector => html! { <TemplateSelector/> },
        Route::TemplateEditor { pane_count } => html! { <TemplateEditor {pane_count}/> },
        Route::NotFound => html! { <div> {"Erro! Página não encontrada. Contate o desenvolvedor"} </div> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

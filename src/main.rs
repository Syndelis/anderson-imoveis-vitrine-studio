#![feature(trait_alias)]

mod editor;
mod selector;
pub mod components;

use editor::TemplateEditor;
use selector::TemplateSelector;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
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
            <div> {"Before"} </div>
            <main>
                <Switch<Route> render={switch} />
            </main>
            <div> {"After"} </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::TemplateSelector => html! { <TemplateSelector/> },
        Route::TemplateEditor { pane_count } => html! { <TemplateEditor {pane_count}/> },
        Route::NotFound => html! { <div> {"Erro! Página não encontrada. Contate o desenvolvedor"} </div> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

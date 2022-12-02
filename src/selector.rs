use yew::prelude::*;

use crate::{Route, components::ButtonLink};

#[function_component]
pub fn TemplateSelector() -> Html {
    
    // let msg = use_context::<MessageContext>().unwrap();
    // let msg = Rc::new(msg);
// 
    // let pane_select_onclick = |pane_count| set_page(Page::TemplateEditor { pane_count }, msg.clone());
    
    html! {
        <main>
            {"Selecionar Template"}
            <ButtonLink<Route> to={Route::Showcase}>
                { "Lançamento" }
            </ButtonLink<Route>>

            <ButtonLink<Route> to={Route::TemplateEditor { pane_count: 1 }}>
                { "Um Imóvel" }
            </ButtonLink<Route>>

            <ButtonLink<Route> to={Route::TemplateEditor { pane_count: 2 }}>
                { "Dois Imóveis" }
            </ButtonLink<Route>>

        </main>
    }
}
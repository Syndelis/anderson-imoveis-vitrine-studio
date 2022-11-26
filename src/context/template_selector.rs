use std::rc::Rc;

use yew::prelude::*;

use crate::context::{MessageContext, Page, set_page};

#[function_component]
pub fn TemplateSelector() -> Html {
    
    let msg = use_context::<MessageContext>().unwrap();
    let msg = Rc::new(msg);

    let pane_select_onclick = |pane_count| set_page(Page::TemplateEditor { pane_count }, msg.clone());
    
    html! {
        <div>
            {"Selecionar Template"}
            <button type="button" onclick={pane_select_onclick(1)}>{"Um Imóvel"}</button>
            <button type="button" onclick={pane_select_onclick(2)}>{"Dois Imóveis"}</button>
        </div>
    }
}
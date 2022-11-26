use yew::prelude::*;

use crate::context::{MessageContext, Page, set_page};

#[derive(Properties, Debug, PartialEq)]
pub struct TemplateEditorProps {
    #[prop_or_default]
    pub pane_count: u32,
}

#[function_component]
pub fn TemplateEditor(props: &TemplateEditorProps) -> Html {
    
    let msg = use_context::<MessageContext>().unwrap();
    let pane_count = props.pane_count;
    
    html! {
        <div>
            {"Template Editor - "}{props.pane_count}

            <div>
                {for (0..pane_count).map(|_| html_nested!(<Template/>))}
            </div>

            <button type="button" onclick={set_page(Page::TemplateSelector, Box::new(msg))}>{"Voltar"}</button>
        </div>
    }
}

#[function_component]
pub fn Template() -> Html {
    html! {
        <div>
            {"Template"}
        </div>
    }
}
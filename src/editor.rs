mod apartment;

use yew::prelude::*;

use crate::Route;
use crate::components::{ButtonLink, ImageInput};

#[derive(Properties, Debug, PartialEq)]
pub struct TemplateEditorProps {
    #[prop_or_default]
    pub pane_count: u32,
}

#[function_component]
pub fn TemplateEditor(props: &TemplateEditorProps) -> Html {
    
    let pane_count = props.pane_count;
    
    html! {
        <div>
            {"Template Editor - "}{pane_count}

            <div>
                {for (0..pane_count).map(|_| html_nested!(<ImageInput/>))}
            </div>

            <ButtonLink<Route> to={Route::TemplateSelector}>
                { "Voltar" }
            </ButtonLink<Route>>
        </div>
    }
}

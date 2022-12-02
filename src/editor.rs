mod apartment;

use yew::prelude::*;

use crate::Route;
use crate::components::BackAndPrint;
use crate::editor::apartment::ApartmentTemplate;

#[derive(Properties, Debug, PartialEq)]
pub struct TemplateEditorProps {
    #[prop_or_default]
    pub pane_count: u32,
}

#[function_component]
pub fn TemplateEditor(props: &TemplateEditorProps) -> Html {
    
    let pane_count = props.pane_count;
    
    html! {
        <main>
            <div class="apartments">
                {for (0..pane_count).map(|_| html_nested!(<ApartmentTemplate {pane_count}/>))}
            </div>

            <BackAndPrint to={Route::TemplateSelector}/>
        </main>
    }
}

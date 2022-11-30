use yew::prelude::*;

use crate::components::ImageInput;

#[derive(Properties, Debug, PartialEq)]
pub struct ApartmentTemplateProps {
    #[prop_or_default]
    pub pane_count: u32,
}

impl ApartmentTemplateProps {
    fn class_name(&self) -> String {
        match self.pane_count {
            1 => "single",
            2 => "double",
            _ => panic!("Invalid pane count"),
        }.into()
    }
}

#[function_component]
pub fn ApartmentTemplate(props: &ApartmentTemplateProps) -> Html {

    let pane_class = props.class_name();

    html! {
        <div class={classes!("row", pane_class.clone())}>
            <ImageInput classes={classes!(pane_class.clone())}/>
            <div>
                <h1 class={classes!("area-of-text", pane_class.clone())} contenteditable="true" data-placeholder="Título"/>
                <p class={classes!("area-of-text", pane_class.clone())} contenteditable="true" data-placeholder="Descrição"/>
                <h1 class={classes!("area-of-text", pane_class)} contenteditable="true" data-placeholder="Preço"/>
            </div>
        </div>
    }
}
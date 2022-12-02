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
            <div class="apartment-text-inputs">
                <div class={classes!("horizontal-aligner", pane_class.clone())}>
                    <h1 class={classes!("area-of-text", pane_class.clone())} contenteditable="true" data-placeholder="Título">
                        { "Ed. De Paulo da Silva Sauro" }
                        <div>{ "Frente Para o Mar" }</div>
                    </h1>
                </div>
                <p class={classes!("area-of-text", pane_class.clone())} contenteditable="true" data-placeholder="Descrição">
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam feugiat aliquet lobortis. Etiam eget dictum libero. Suspendisse eget sapien consequat, dignissim nisi vitae, ullamcorper arcu. Quisque at iaculis lorem. Duis imperdiet pulvinar nisi, at congue lorem adasd" }
                </p>
                <div class={classes!("horizontal-aligner", pane_class.clone())}>
                    <h1 class={classes!("area-of-text", pane_class)} contenteditable="true" data-placeholder="Preço">
                        { "VALOR R$ 3.400.000,00 À VISTA" }
                    </h1>
                </div>
            </div>
        </div>
    }
}
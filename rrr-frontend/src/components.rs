use yew::prelude::*;
use yew_hooks::prelude::*;
use material_yew::*;

#[derive(Properties, PartialEq)]
pub struct ChildrenProps {
    pub children: Children,
}

#[function_component]
pub fn HorizontalLayout(props: &ChildrenProps) -> Html {
    html!(
        <div class="horizontal-layout">
            {props.children.clone()}
        </div>
    )
}

#[function_component]
pub fn VerticalLayout(props: &ChildrenProps) -> Html {
    html!(
        <div class="vertical-layout">
            {props.children.clone()}
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    pub title: String,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html!(
        <div class="card">
            <div class="header">
                if(props.icon.is_some()) {<MatIcon>{props.icon.clone().unwrap()}</MatIcon>}
                <h2>{props.title.clone()}</h2>
            </div>
            <div class="card-content">{props.children.clone()}</div>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct TabPageProps {
    pub children: Children,
    pub id: usize,
    pub current_id: usize,
}

#[function_component]
pub fn TabPage(props: &TabPageProps) -> Html {
    html! {
        <div class="tab-page" hidden={props.id != props.current_id}>{props.children.clone()}</div>
    }
}
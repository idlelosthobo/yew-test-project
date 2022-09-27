use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub number: i32,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    html! {
        <div class="border border-1">
            { &props.number }
        </div>
    }
}
use yew::prelude::*;

#[function_component(Cell)]
pub fn cell() -> Html {
    html! {
        <div class="border border-1">
            {"Cell"}
        </div>
    }
}
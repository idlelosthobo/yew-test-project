use yew::prelude::*;

#[function_component(TopNavigation)]
pub fn top_navigation() -> Html {
    let _number: i32 = 20;
    html! {
        <div class="row">
            <div class="col-12 sticky-top">
            {"Navigations"}
            <a href="#">{"TEdious"}</a>
            </div>
        </div>
    }
}
pub mod nav;
pub mod game;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container-fluid">
            <nav::TopNavigation/>
            <game::Game/>
        </div>
    }
}
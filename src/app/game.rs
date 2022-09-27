mod puzzle;

use yew::prelude::*;

#[function_component(Game)]
pub fn game() -> Html {
    html! {
        <div class="row">
            <div class="col-12">
                <puzzle::PuzzleBoard/>
            </div>
        </div>
    }
}
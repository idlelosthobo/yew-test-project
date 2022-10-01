mod puzzle;

use yew::prelude::*;

#[function_component(GameComponent)]
pub fn game_component() -> Html {
    html! {
        <div class="row">
            <div class="col-12">
                <puzzle::PuzzleComponent puzzle={puzzle::Puzzle {size: 9}}/>
            </div>
        </div>
    }
}
mod cell;

use yew::{prelude::*, props};

#[derive(Properties, PartialEq)]
struct Puzzle {
    size: i32,
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board() -> Html {
    let mut tacos = vec![];
    for i in 0..18 {
        tacos.push(cell::CellProps {number: i});
    }
    let videos = tacos.iter().map(|cell_data| html! {
        <cell::Cell number={cell_data.number} />
    }).collect::<Html>();
    html! {
        <>
            {videos}
        </>
    }
}
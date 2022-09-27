mod cell;

use yew::{prelude::*, props};

#[derive(Properties, PartialEq)]
struct Puzzle {
    size: i32,
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board() -> Html {
    let tacos = vec![
        cell::CellProps {number: 1},
        cell::CellProps {number: 2},
        cell::CellProps {number: 3},
        cell::CellProps {number: 4},
        cell::CellProps {number: 5},
        ];
    let videos = tacos.iter().map(|cell_data| html! {
        <cell::Cell number={cell_data.number} />
    }).collect::<Html>();
    html! {
        <>
            {videos}
        </>
    }
}
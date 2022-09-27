mod cell;

use yew::prelude::*;

struct Puzzle {
    size: i32,
}

fn generate_puzzle(size: i32) -> Puzzle {
    return Puzzle { size };
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board() -> Html {
    let _puzzle = generate_puzzle(9);
    let _puzzle_list: Vec<cell::Cell> = Vec::new();

    for i in 0.._puzzle.size {
        _puzzle_list.push(cell::Cell<Html>)
    }

    html! {
        <>
            { for _puzzle_list.iter() }
        </>
    }
}
mod cell;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Puzzle {
    pub size: i32
}

impl Puzzle {
    fn grid_size(&self) -> i32 {
        self.size * self.size
    }
}

#[derive(Properties, PartialEq)]
pub struct PuzzleProps {
    pub puzzle: Puzzle,
}

#[function_component(PuzzleComponent)]
pub fn puzzle_component(props: &PuzzleProps) -> Html {
    let mut tacos = vec![];
    for i in 0..props.puzzle.grid_size() {
        tacos.push(cell::Cell { number: i, cell_size: (100 / props.puzzle.size) });
    }
    let cells = tacos.iter().map(|cell_data| html! {
        <cell::CellComponent cell={cell_data.clone()} />
    }).collect::<Html>();
    html! {
        <div class="row px-3">
            {cells}
        </div>
    }
}
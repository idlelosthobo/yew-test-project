use std::fmt::format;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Cell {
    pub number: i32,
    pub cell_size: i32,
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellProps) -> Html {
    let style = format!("width: {}%;", props.cell.cell_size);
    html! {
        <div class="col-auto border border-1" style={style}>
            { &props.cell.number }
        </div>
    }
}
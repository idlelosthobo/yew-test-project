mod cell;

use yew::prelude::*;

pub struct Puzzle {
    pub size: i32,
}

pub enum Msg {
}

impl Puzzle {
    fn grid_size(&self) -> i32 {
        self.size * self.size
    }
}

impl Component for Puzzle {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Puzzle {
            size: 9,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut tacos = vec![];

        for i in 0..self.grid_size() {
            tacos.push(cell::CellProps { number: i, cell_size: (100 / self.size), selected: false });
        }

        let cells = tacos.iter().map(|cell_data| html! {
            <cell::Cell ..cell_data.clone() />
        }).collect::<Html>();

        html! {
            <div class="row px-3">
                {cells}
        </div>
        }
    }
}
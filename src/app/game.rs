mod puzzle;

use yew::prelude::*;

pub struct Game {
    pub time_elapsed: i64,
}

pub enum Msg {
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Game {
            time_elapsed: 0,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                <div class="col-12">
                    <puzzle::Puzzle/>
                </div>
            </div>
        }
    }
}


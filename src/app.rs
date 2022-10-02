pub mod nav;
pub mod game;

use yew::prelude::*;

pub struct App {
    counter: i32,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            counter: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container-fluid">
                <nav::TopNavigation/>
                <game::Game/>
            </div>
        }
    }
}

use yew::prelude::*;

pub struct Cell {
    pub number: i32
}


#[derive(Clone, PartialEq, Properties)]
pub struct CellProps {
    pub number: i32,
    pub cell_size: i32,
    pub selected: bool,
}

pub enum Msg {
    Clicked
}

impl Cell {
}

impl Component for Cell {
    type Message = Msg;
    type Properties = CellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            number: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.number += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!("width: {}%; height: {}%;", ctx.props().cell_size, ctx.props().cell_size);
        html! {
            <div class="col-auto border border-1" style={style} onclick={ctx.link().callback(|_| Msg::Clicked)}>
                { ctx.props().number }
                <br />
                { self.number }
            </div>
        }
    }

}


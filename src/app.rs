mod nav;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <nav::TopNavigation></nav::TopNavigation>
            <div class="row">
                <div class="col-3">
                {"Neato"}
                </div>
            </div>
            <h1>{"Basic Yew Web App Update NEATO"}</h1>
            <h2>{"CLassic"}</h2>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
            <h4>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Amet, neque in distinctio, aliquam rerum expedita reprehenderit libero nobis cupiditate minus delectus possimus repellat ipsa? Aperiam maiores perferendis voluptatem consectetur ab."}</h4>
        </div>
    }
}
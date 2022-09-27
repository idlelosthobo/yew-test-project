use yew::prelude::*;

#[function_component(TopNavigation)]
pub fn top_navigation() -> Html {
    let _number: i32 = 20;
    html! {
        <div class="row sticky-top">
            <div class="col-12 px-0">
                <nav class="navbar bg-light">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="#">{"KenKen"}</a>
                    </div>
                </nav>
            </div>
        </div>
    }
}

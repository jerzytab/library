use yew::prelude::*;

use components::header::Header;
use components::book::Book;
use components::nav::Nav;
mod components;

#[function_component]
fn App() -> Html {
    html! {
        <div style="background-color: #e8ecda;" class=" h-screen">
            <Header />
            <Nav />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
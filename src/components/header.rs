use yew::prelude::*;

#[component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1>{ "Welcome to Napp!" }</h1>
        </header>
    }
}

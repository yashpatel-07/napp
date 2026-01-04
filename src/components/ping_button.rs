use crate::services::ping_service::spawn_ping;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PingButtonProps {
    pub url: String,
    pub on_ping: Callback<f64>,
}

#[function_component(PingButton)]
pub fn ping_button(props: &PingButtonProps) -> Html {
    let on_click = {
        let url = props.url.clone();
        let on_ping = props.on_ping.clone();
        Callback::from(move |_| {
            let on_ping = on_ping.clone();
            spawn_ping(url.clone(), move |duration| {
                on_ping.emit(duration);
            });
        })
    };

    html! {
        <div class="flex-container">
        <button class="btn btn-primary" onclick={on_click}>{ format!("Ping {}", props.url) }</button>
        </div>
    }
}

use yew::prelude::*;

use crate::components::ping_button::PingButton;
use crate::components::ping_list::PingList;
use crate::types::ping_result::PingResult;

#[component]
pub fn App() -> Html {
    let results = use_state(|| vec![]);

    let add_result = {
        let results = results.clone();
        Callback::from(move |duration: f64| {
            let mut new_results = (*results).clone();
            new_results.push(PingResult {
                url: "https://example.com".into(),
                duration_ms: duration,
            });
            results.set(new_results);
        })
    };

    html! {
        <div>
            <h1>{ "WASM Ping App" }</h1>
            <PingButton url={"https://example.com"} on_ping={add_result.clone()} />
            <PingList results={(*results).clone()} />
        </div>
    }
}

use crate::types::ping_result::PingResult;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PingListProps {
    pub results: Vec<PingResult>,
}

#[function_component(PingList)]
pub fn ping_list(props: &PingListProps) -> Html {
    html! {
        <div class="flex-container">
            <ul>
                { for props.results.iter().map(|r| html!{
                    <li class="list-item">{ format!("{} → {:.2} ms", r.url, r.duration_ms) }</li>
                }) }
            </ul>
        </div>
    }
}

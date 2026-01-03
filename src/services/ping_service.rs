use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

/// Ping a URL and return duration in milliseconds
pub async fn ping_url(url: &str) -> f64 {
    let start = js_sys::Date::now();
    let _ = Request::get(url).send().await;
    js_sys::Date::now() - start
}

/// Utility to run ping and call a callback (for Yew)
pub fn spawn_ping<F: Fn(f64) + 'static>(url: String, callback: F) {
    spawn_local(async move {
        let duration = ping_url(&url).await;
        callback(duration);
    });
}
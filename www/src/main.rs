use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::Request;
use yew::{function_component, Html, use_effect, html};

fn main() {
    yew::Renderer::<App>::new().render();
}

struct Point {
    x: i32,
    y: i32,
    color: i32,
}

#[function_component]
fn App() -> Html {
    use_effect(move ||{
        let canvas = document()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas.get_context("2d").unwrap().unwrap();
        let context_2d = context
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let timers = gloo::timers::callback::Interval::new(5000, move || {
            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest::get("http://localhost/grid").await.unwrap();
            })
        });
        timers.forget();
    });

    html! {
        <div>
            <canvas id="canvas" width="4000" height="4000"></canvas>
        </div>
    }
}

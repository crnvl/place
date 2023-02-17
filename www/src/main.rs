use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use yew::{function_component, html, use_effect, Html};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
    color: i32,
}

#[function_component]
fn App() -> Html {
    use_effect(move || {
        let timers = gloo::timers::callback::Interval::new(1000, || {
            wasm_bindgen_futures::spawn_local(async {
                let canvas = document()
                    .get_element_by_id("canvas")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();

                let context = canvas.get_context("2d").unwrap().unwrap();
                let context_2d = context
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let res = reqwest::get("http://localhost/grid").await.unwrap();
                res.json::<Vec<Point>>()
                    .await
                    .unwrap()
                    .iter()
                    .for_each(|point| {
                        context_2d.set_fill_style(&JsValue::from_str(&format!(
                            "rgb({},{},{})",
                            point.color, point.color, point.color
                        )));
                        context_2d.fill_rect(
                            (point.x * 10) as f64,
                            (point.y * 10) as f64,
                            10.,
                            10.,
                        );
                    });
            })
        });
        timers.forget();
    });

    html! {
        <>
            <div>
                <canvas id="canvas" width="4000" height="4000"></canvas>
            </div>
        </>
    }
}

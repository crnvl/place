use gloo::utils::document;
use wasm_bindgen::{JsCast, JsValue};
use yew::{function_component, html, use_effect, Html};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    use_effect(move || {
        let canvas = document()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas.get_context("2d").unwrap().unwrap();
        let context_2d = context
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context_2d.set_fill_style(&JsValue::from_str("red"));
        context_2d.fill_rect(0.0, 0.0, 500.0, 500.0);

        context_2d.set_fill_style(&JsValue::from_str("blue"));
        context_2d.fill_rect(100.0, 100.0, 500.0, 500.0);

        context_2d.set_fill_style(&JsValue::from_str("green"));
        context_2d.fill_rect(200.0, 200.0, 500.0, 500.0);
    });

    html! {
        <div>
            <canvas id="canvas" width="4000" height="4000"></canvas>
        </div>
    }
}

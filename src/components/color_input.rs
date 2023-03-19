use crate::color::Color;

use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use gloo::events::EventListener;
use gloo_console::log;
use gloo_timers::callback::Timeout;

#[derive(Debug)]
enum ColorInputState {
    MouseUp,
    SquareSelected,
    HueSelected
}

#[derive(Properties, PartialEq)]
pub struct ColorInputProps {
    pub text: String,
    pub color: Color,
    pub onchange: Callback<Color>
}

#[function_component(ColorInput)]
pub fn color_input(props: &ColorInputProps) -> Html {
    let state = use_state(|| ColorInputState::MouseUp);
    let color_picker_enable = use_state(|| false);
    let slider_x = use_state(|| 0);

    let black_square = use_node_ref();
    let hue_selector = use_node_ref();
    let color_picker = use_node_ref();

    let color = props.color;

    let square_mousedown = {
        let state = state.clone();
        move |_| {
            state.set(ColorInputState::SquareSelected);
        }
    };

    let hue_mousedown = {
        let state = state.clone();
        move |_| {
            state.set(ColorInputState::HueSelected);
        }
    };

    
    // add mousemove event listener to document window
    use_effect({
        let state = state.clone();
        let color = color.clone();
        let slider_x = slider_x.clone();
        let onchange = props.onchange.clone();

        let window = web_sys::window().expect("Could not get window!");
        let black_square = black_square.clone();
        let hue_selector = hue_selector.clone();
        move || {
            let mousemove = {
                let state = state.clone();
                let color = color.clone();
                let slider_x = slider_x.clone();
                let onchange = onchange.clone();

                let black_square = black_square.clone();
                let hue_selector = hue_selector.clone();

                Callback::from(move |event: MouseEvent| {
                    let slider_x = slider_x.clone();

                    match *state {
                        ColorInputState::SquareSelected => {
                            let black_square = black_square.clone().cast::<HtmlElement>().unwrap();
                            let rect = black_square.get_bounding_client_rect();
        
                            let x = event.page_x() - rect.x() as i32;
                            let y = event.page_y() - rect.y() as i32;
        
        
                            let x = x.clamp(0, black_square.client_width());
                            let y = y.clamp(0, black_square.client_height());
        
                            let saturation = x as f32 / black_square.client_width() as f32;
                            let value = 1.0 - (y as f32 / black_square.client_height() as f32);
        
                            onchange.emit(color.update_value(value).update_saturation(saturation));
                        },
                        ColorInputState::HueSelected => {
                            let hue_selector = hue_selector.clone().cast::<HtmlElement>().unwrap();
                            let rect = hue_selector.get_bounding_client_rect();
                            let x = event.page_x() - rect.x() as i32;
                            let x_clamp = x.clamp(0, rect.width() as i32);
        
                            let hue = x_clamp as f32 / rect.width() as f32;

                            onchange.emit(color.update_hue(hue));
                            slider_x.set(x_clamp.min(rect.width() as i32 - 10));
                        },
                        ColorInputState::MouseUp => {}
                    }
                })
            };
        
            let mousemove_listener = EventListener::new(
                &window,
                "mousemove",
                {
                    let mousemove = mousemove.clone();
                    move |e| mousemove.emit(e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw().clone())
                }
            );

            let mousedown_listener = EventListener::new(
                &window,
                "mousedown",
                move |e| mousemove.clone().emit(e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw().clone())
            );
        
            move || { drop(mousemove_listener); drop(mousedown_listener) }

        }
    });

    // add mouseup event listener to document window
    use_effect({
        let state = state.clone();
        let color_picker_enable = color_picker_enable.clone();

        let window = web_sys::window().expect("Could not get window!");
        let color_picker = color_picker.clone();

        move || {
            let color_picker = color_picker.clone();
            let mouseup = {
                let state = state.clone();
                let color_picker_enable = color_picker_enable.clone();
        
                Callback::from(move |e: MouseEvent| {
                    state.set(ColorInputState::MouseUp);

                    // check if mouse event is within color_picker
                    if *color_picker_enable {
                        let rect = color_picker.cast::<HtmlElement>().expect("color_picker cast failed!").get_bounding_client_rect();
                        
                        let mouse_x = e.page_x() as f64;
                        let mouse_y = e.page_y() as f64;
    
                        if (mouse_x < rect.x()) ||
                          (mouse_y < rect.y()) ||
                          (mouse_x > (rect.x() + rect.width())) ||
                          (mouse_y > (rect.y() + rect.height())) {
                            Timeout::new(10, { 
                                let color_picker_enable = color_picker_enable.clone(); 
                                move || color_picker_enable.set(false) 
                            }).forget();
                        }
                    }
                })
            };
        
            let listener = EventListener::new(
                &window,
                "mouseup",
                move |e| mouseup.emit(e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw().clone())
            );
        
            move || drop(listener)
        }
    });

    let onclick = {
        let color_picker_enable = color_picker_enable.clone();
        move |_| { color_picker_enable.set(!*color_picker_enable) }
    };

    html! {
        <div>
            <button {onclick}>
                {&props.text} 
                <span class="color-circle" style={format!("background: {}", &color.to_hex())}></span>
            </button>

            if *color_picker_enable {
                <div class="color-picker" ref={color_picker}>
                    <div 
                        class="base-square" 
                        style={
                            format!("background: {}", &color.update_saturation(1.0).update_value(1.0).to_hex())
                        }
                    >
                        <div class="white-square">
                            <div 
                                class="black-square" 
                                ref={black_square}
                                onmousedown={square_mousedown}
                            ></div>
                        </div>
                    </div>
                    <div 
                        class="hue-selector"
                        ref={hue_selector}
                        onmousedown={hue_mousedown}
                    >
                        <div class="hue-slider" style={format!("margin-left: {}px", *slider_x)}></div>
                    </div>
                </div>
            }
        </div>
    }
}

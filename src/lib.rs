mod color;
mod components;

use std::num;

use components::*;
use color::Color;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_console::log;
use qrcode::QrCode;
use qrcode::render::svg;
use qrcode::types::{EcLevel, QrError};
use base64::{Engine as _, engine::general_purpose};
use image::{Rgb, ImageOutputFormat, DynamicImage};
use web_sys::{HtmlAnchorElement, HtmlInputElement};

#[derive(Properties, PartialEq)]
struct OptionsProps {
    eclevel: EcLevel,
    foreground_color: Color,
    background_color: Color,
    change_eclevel: Callback<EcLevel>,
    dispatch: Callback<AppAction>
}

#[function_component(Options)]
fn options(props: &OptionsProps) -> Html {
    let options_visible = use_state(|| false);

    let change_ec = {
        let dispatch = props.dispatch.clone();
        move |level| {
            let dispatch = dispatch.clone();
            move |_| {
                dispatch.emit(AppAction::UpdateEcLevel(level));
            }
        }
    };

    let visible_onclick = {
        let options_visible = options_visible.clone();
        move |_| {
            options_visible.set(!*options_visible);
        }
    };
    
    let button_message = if *options_visible {
        "Hide Advanced Options"
    } else {
        "Show Advanced Options"
    };
    
    html! {
        <>
            if *options_visible {
                <div class="advanced-options">
                    <div class="change-ec">
                        { "Error correction level: " }
                        <input type="radio" name="eclevel" id="L" onclick={ change_ec(EcLevel::L) } checked={ props.eclevel == EcLevel::L }/>
                        <label for="L">{ "7%" }</label>
                        <input type="radio" name="eclevel" id="M" onclick={ change_ec(EcLevel::M) } checked={ props.eclevel == EcLevel::M }/>
                        <label for="M">{ "15%" }</label>
                        <input type="radio" name="eclevel" id="Q" onclick={ change_ec(EcLevel::Q) } checked={ props.eclevel == EcLevel::Q }/>
                        <label for="Q">{ "25%" }</label>
                        <input type="radio" name="eclevel" id="H" onclick={ change_ec(EcLevel::H) } checked={ props.eclevel == EcLevel::H }/>
                        <label for="H">{ "30%" }</label>
                    </div>
                    <div class="change-color">
                        <ColorInput 
                            text="Background Color"
                            color={props.background_color}
                            onchange={
                                let dispatch = props.dispatch.clone();
                                move |color| dispatch.emit(AppAction::UpdateBackgroundColor(color))
                            }
                        />
                        <ColorInput 
                            text="Foreground Color"
                            color={props.foreground_color}
                            onchange={
                                let dispatch = props.dispatch.clone();
                                move |color| dispatch.emit(AppAction::UpdateForegroundColor(color))
                            }
                        />
                    </div>
                </div>
            }
            <input type="button" value={button_message} onclick={visible_onclick}/>
        </>
    }
}

enum AppAction {
    UpdateEcLevel(EcLevel),
    CloseError,
    GenerateQrCode(Vec<u8>),
    UpdateBackgroundColor(Color),
    UpdateForegroundColor(Color),
}

#[derive(Clone, Debug)]
struct QrInfo {
    data: Vec<u8>,
    svg: String
}

#[derive(Clone, Debug)]
struct AppState {
    qr: Option<QrInfo>,
    error: Option<String>,
    ec_level: EcLevel,
    foreground_color: Color,
    background_color: Color,
}

impl Reducible for AppState {
    type Action = AppAction;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppAction::CloseError => {
                AppState { error: None, ..(*self).clone() }.into()
            },
            AppAction::GenerateQrCode(data) => {
                match QrCode::with_error_correction_level(&data, *&self.ec_level) {
                    Ok(code) => {
                        let svg = code.render::<svg::Color>()
                            .light_color(svg::Color(&self.background_color.to_hex()))
                            .dark_color(svg::Color(&self.foreground_color.to_hex()))
                            .max_dimensions(1, 1)
                            .build();
                        AppState { qr: Some(QrInfo { data, svg }), ..(*self).clone() }
                    },
                    Err(QrError::DataTooLong) => {
                        let message = String::from("Data is too large! (2,331 max bytes or 3,391 max alphanumeric characters)");
                        AppState { error: Some(message), ..(*self).clone() }
                    },
                    Err(e) => {
                        let message = format!("An error has occured. ({e:?})");
                        AppState { error: Some(message), ..(*self).clone() }
                    }
                }.into()
            },
            AppAction::UpdateEcLevel(ec_level) => {
                let new_state = AppState { ec_level, ..(*self).clone() };
                if let Some(qr_info) = self.qr.clone() {
                    AppState::reduce(new_state.into(), AppAction::GenerateQrCode(qr_info.data))
                } else {
                    new_state.into()
                }
            },
            AppAction::UpdateBackgroundColor(background_color) => {
                let new_state = AppState { background_color, ..(*self).clone() };
                if let Some(qr_info) = self.qr.clone() {
                    AppState::reduce(new_state.into(), AppAction::GenerateQrCode(qr_info.data))
                } else {
                    new_state.into()
                }
            },
            AppAction::UpdateForegroundColor(foreground_color) => {
                let new_state = AppState { foreground_color, ..(*self).clone() };
                if let Some(qr_info) = self.qr.clone() {
                    AppState::reduce(new_state.into(), AppAction::GenerateQrCode(qr_info.data))
                } else {
                    new_state.into()
                }
            },
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| AppState { 
        qr: None, 
        error: None, 
        ec_level: EcLevel::M,
        foreground_color: Color::from_rgb(0.0, 0.0, 0.0),
        background_color: Color::from_rgb(1.0, 1.0, 1.0),
    });
    let svg = state.qr.as_ref().map(|qr| qr.svg.clone());
    let link_ref = use_node_ref();

    let dispatch = {
        let state = state.clone();
        move |action| {
            state.dispatch(action)
        }
    };

    let error_close = {
        let state = state.clone();
        move |_| {
            state.dispatch(AppAction::CloseError)
        }
    };

    let generate = {
        let state = state.clone();
        move |data: Vec<u8>| {
            state.dispatch(AppAction::GenerateQrCode(data))
        }
    };

    let change_eclevel = {
        let state = state.clone();
        move |ec_level| {
            state.dispatch(AppAction::UpdateEcLevel(ec_level))
        }
    };

    #[derive(Clone, Copy)]
    enum SaveType { Svg, Jpeg, Png }

    impl TryInto<ImageOutputFormat> for SaveType {
        type Error = ();
        fn try_into(self) -> Result<ImageOutputFormat, ()> {
            match self {
                SaveType::Png => Ok(ImageOutputFormat::Png),
                SaveType::Jpeg => Ok(ImageOutputFormat::Jpeg(70)),
                _ => Err(())
            }
        }
    }

    impl SaveType {
        fn to_mime(self) -> String {
            String::from(
                match self {
                    SaveType::Png => "image/png",
                    SaveType::Jpeg => "image/jpeg",
                    SaveType::Svg => "image/svg+xml",
                }
            )
        }

        fn to_filename(self) -> String {
            String::from(
                match self {
                    SaveType::Png => "qr.png",
                    SaveType::Jpeg => "qr.jpeg",
                    SaveType::Svg => "qr.svg",
                }
            )
        }
    }

    let save = {
        let state = state.clone();
        let link_ref = link_ref.clone();

        fn to_array<T>(xs: (T, T, T)) -> [T; 3] { [xs.0, xs.1, xs.2] }
        
        move |save_type| {
            move |_: MouseEvent| {
                if let Some(qr) = &state.qr {
                    let link = link_ref.cast::<HtmlAnchorElement>().expect("input_ref not bound to anchor!");
                    let data = match save_type {
                        SaveType::Svg => general_purpose::STANDARD_NO_PAD.encode(&qr.svg),
                        save_type => {
                            let mut buffer = Vec::new();
                            let foreground_color = state.foreground_color.to_rgb_u8();
                            let background_color = state.background_color.to_rgb_u8();
                            let format: ImageOutputFormat = save_type.try_into().unwrap();
                            DynamicImage::ImageRgb8(QrCode::new(&qr.data).unwrap().render::<Rgb<u8>>()
                                .light_color(Rgb(to_array(background_color)))
                                .dark_color(Rgb(to_array(foreground_color)))
                                .max_dimensions(200, 200)
                                .build()).write_to(&mut buffer, format).expect("Failed to write image to buffer");

                            general_purpose::STANDARD_NO_PAD.encode(&buffer)
                        }
                    };

                    link.set_href(&format!("data:{};base64,{}", save_type.to_mime(), data));
                    link.set_download(&save_type.to_filename());
                    link.click();
                }
            }
        }
    };

    html! {
        <>
        <div class="main-container">
        <main>
            <a style="diplay: none;" ref={link_ref}></a>
            <h1>{ "QR Code Generator" }</h1>
            <QrOutput qr={svg}/>
            if state.qr.is_some() {
                <div class="save-buttons">
                    <input type="button" value="Save as PNG" onclick={ let save = save.clone(); save(SaveType::Png) }/>
                    <input type="button" value="Save as JPEG" onclick={ let save = save.clone(); save(SaveType::Jpeg) }/>
                    <input type="button" value="Save as SVG" onclick={ let save = save.clone(); save(SaveType::Svg)}/>
                </div>
            }
            if let Some(message) = &state.error {
                <ErrorPopup message={message.clone()} close={error_close} />
            }
            <Options 
                eclevel={*&state.ec_level}
                foreground_color={*&state.foreground_color}
                background_color={*&state.background_color}
                {change_eclevel}
                {dispatch}
            />
            <TextInput generate={generate.clone()}/>
            <FileInput {generate}/>
        </main>
        </div>
        <footer><a href="https://github.com/RainbowAsteroids/qr-portal2d" target="_blank" rel="noopener noreferrer">{ "Source code" }</a></footer>
        </>
    }
}

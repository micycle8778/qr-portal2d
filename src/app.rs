use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlAnchorElement};
use gloo_console::log;
use gloo::file::{callbacks, File};
use gloo::file::callbacks::FileReader;
use qrcode::QrCode;
use qrcode::render::svg;
use qrcode::types::QrError;
use base64::{Engine as _, engine::general_purpose};
use image::{Rgb, ImageOutputFormat, DynamicImage};

#[derive(Properties, PartialEq)]
struct QrProps {
    qr: Option<String>
}

#[function_component(QrOutput)]
fn qr_output(props: &QrProps) -> Html {
    match &props.qr {
        None => html! {<div class="qr-output">{ "No QR code has been generated." }</div>},
        Some(qr) => { 
            let data = general_purpose::STANDARD_NO_PAD.encode(qr);
            html! {
                <div class="qr-output">
                    <img src={format!("data:image/svg+xml;base64,{data}")}/>
                </div>
            }
        }
    }
}

#[derive(Properties, PartialEq)]
struct TextInputProps {
    generate: Callback<Vec<u8>>
}

#[function_component(TextInput)]
fn text_input(props: &TextInputProps) -> Html {
    let input_ref = use_node_ref();

    let onclick = { 
        let input_ref = input_ref.clone();
        let event = props.generate.clone();
        
        move |_| {
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to text input!");
            let text = input.value();
            log!(&text);
            event.emit(text.into());
            input.set_value("");
        }
    };
    
    let onkeypress = { 
        let input_ref = input_ref.clone();
        let event = props.generate.clone();
        
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to text input!");
                let text = input.value();
                log!(&text);
                event.emit(text.into());
                input.set_value("");
            }
        }
    };
    
    html! {
        <div class="textinput">
            <input type="text" placeholder="Paste text, URL, etc." {onkeypress} ref={input_ref}/>
            <input type="button" value="Generate" {onclick}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FileInputProps {
    generate: Callback<Vec<u8>>
}

#[function_component(FileInput)]
fn file_input(props: &FileInputProps) -> Html {
    let input_ref = use_node_ref();
    // We need to store the FileReader as it reads the file, else the read will be cancelled.
    let reader = use_state(|| None::<FileReader>);

    let onchange = {
        let input_ref = input_ref.clone();
        let event = props.generate.clone();
        let reader = reader.clone();
        
        move |_| {
            log!("onchange");
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to input!");
            let file_list = input.files().expect("input_ref not bound to file input!");
            let event = event.clone();
            let reader = reader.clone();
            log!(&file_list);
            
            if let Some(file) = file_list.get(0) {
                log!("Some(file)");
                let task = callbacks::read_as_bytes(&File::from(file), move |data| {
                    log!("read_as_bytes");
                    event.emit(data.expect("Error reading file"));
                });
                // We need to store the FileReader as it reads the file, else the read will be cancelled.
                reader.set(Some(task));
                
            }
        }
    };

    let onclick = {
        let input_ref = input_ref.clone();
        
        move |_| {
            log!("onclick");
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to input!");
            input.click();
        }
    };
    
    html! {
        <>
            <input type="file" ref={input_ref} style="display: none;" {onchange}/>
            <input type="button" value="Generate QR code from file" {onclick}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ErrorPopupProps {
    message: String,
    close: Callback<()>
}

#[function_component(ErrorPopup)]
fn error_popup(props: &ErrorPopupProps) -> Html {
    let onclick = {
        let event = props.close.clone();
        move |_| {
            event.emit(());
        }
    };
    
    html! {
        <div class="error">
            <div class="error-container">
                <p>{ &props.message }</p>
                <input type="button" value="Close" {onclick}/>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct QrInfo {
    data: Vec<u8>,
    svg: String
}

struct AppState {
    qr: Option<QrInfo>,
    error: Option<String>
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState { qr: None, error: None } );
    let svg = state.qr.as_ref().map(|qr| qr.svg.clone());
    let link_ref = use_node_ref();

    let error_close = {
        let state = state.clone();
        move |_| {
            state.set( AppState { error: None, qr: state.qr.clone() } );
        }
    };

    let generate = {
        let state = state.clone();
        move |data: Vec<u8>| {
            match QrCode::new(&data) {
                Ok(code) => {
                    let svg = code.render::<svg::Color>().build();
                    state.set(AppState { qr: Some(QrInfo { data, svg }), error: state.error.clone() });
                },
                Err(QrError::DataTooLong) => {
                    let message = String::from("Data is too large! (2,331 max bytes or 3,391 max alphanumeric characters)");
                    state.set(AppState { error: Some(message), qr: state.qr.clone() });
                },
                Err(e) => {
                    let message = format!("An error has occured. ({e:?})");
                    state.set(AppState { error: Some(message), qr: state.qr.clone() });
                }
            }
        }
    };

    let save_svg = {
        let state = state.clone();
        let link_ref = link_ref.clone();
        move |_| {
            if let Some(qr) = &state.qr {
                let link = link_ref.cast::<HtmlAnchorElement>().expect("input_ref not bound to anchor!");
                let data = general_purpose::STANDARD_NO_PAD.encode(&qr.svg);
                link.set_href(&format!("data:image/svg+xml;base64,{}", data));
                link.set_download("qr.svg");
                link.click();
            }
        }
    };

    let save_png = {
        let state = state.clone();
        let link_ref = link_ref.clone();
        move |_| {
            if let Some(qr) = &state.qr {
                let link = link_ref.cast::<HtmlAnchorElement>().expect("input_ref not bound to anchor!");

                let mut buffer = Vec::new();
                DynamicImage::ImageRgb8(QrCode::new(&qr.data).unwrap().render::<Rgb<u8>>()
                    .build()).write_to(&mut buffer, ImageOutputFormat::Png).expect("Failed to write image to buffer");
                
                let data = general_purpose::STANDARD_NO_PAD.encode(&buffer);
                link.set_href(&format!("data:image/png;base64,{}", data));
                link.set_download("qr.png");
                link.click();
            }
        }
    };

    let save_jpeg = {
        let state = state.clone();
        let link_ref = link_ref.clone();
        move |_| {
            if let Some(qr) = &state.qr {
                let link = link_ref.cast::<HtmlAnchorElement>().expect("input_ref not bound to anchor!");

                let mut buffer = Vec::new();
                DynamicImage::ImageRgb8(QrCode::new(&qr.data).unwrap().render::<Rgb<u8>>()
                    .build()).write_to(&mut buffer, ImageOutputFormat::Jpeg(70)).expect("Failed to write image to buffer");
                
                let data = general_purpose::STANDARD_NO_PAD.encode(&buffer);
                link.set_href(&format!("data:image/jpeg;base64,{}", data));
                link.set_download("qr.jpeg");
                link.click();
            }
        }
    };

    html! {
        <main>
            <a style="diplay: none;" ref={link_ref}></a>
            <h1>{ "QR Code Generator" }</h1>
            <QrOutput qr={svg}/>
            if state.qr.is_some() {
                <div class="save-buttons">
                    <input type="button" value="Save as PNG" onclick={save_png}/>
                    <input type="button" value="Save as JPEG" onclick={save_jpeg}/>
                    <input type="button" value="Save as SVG" onclick={save_svg}/>
                </div>
            }
            if let Some(message) = &state.error {
                <ErrorPopup message={message.clone()} close={error_close} />
            }
            <TextInput generate={generate.clone()}/>
            <FileInput {generate}/>
        </main>
    }
}

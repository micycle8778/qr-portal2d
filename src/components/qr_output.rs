use yew::prelude::*;
use base64::{Engine as _, engine::general_purpose};

#[derive(Properties, PartialEq)]
pub struct QrProps {
    pub qr: Option<String>
}

#[function_component(QrOutput)]
pub fn qr_output(props: &QrProps) -> Html {
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

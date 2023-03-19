use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorPopupProps {
    pub message: String,
    pub close: Callback<()>
}

#[function_component(ErrorPopup)]
pub fn error_popup(props: &ErrorPopupProps) -> Html {
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


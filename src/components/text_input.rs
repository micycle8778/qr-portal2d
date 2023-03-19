use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub generate: Callback<Vec<u8>>
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let input_ref = use_node_ref();

    let onclick = { 
        let input_ref = input_ref.clone();
        let event = props.generate.clone();
        
        move |_| {
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to text input!");
            let text = input.value();
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
                event.emit(text.into());
                input.set_value("");
            }
        }
    };
    
    html! {
        <div class="textinput">
            <input type="text" placeholder="Type or paste text, URL, etc." {onkeypress} ref={input_ref}/>
            <input type="button" value="Generate" {onclick}/>
        </div>
    }
}

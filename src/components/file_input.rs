use yew::prelude::*;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use gloo::file::{callbacks, File};

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub generate: Callback<Vec<u8>>
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let input_ref = use_node_ref();
    // We need to store the FileReader as it reads the file, else the read will be cancelled.
    let reader = use_state(|| None::<FileReader>);

    let onchange = {
        let input_ref = input_ref.clone();
        let event = props.generate.clone();
        let reader = reader.clone();
        
        move |_| {
            let input = input_ref.cast::<HtmlInputElement>().expect("input_ref not bound to input!");
            let file_list = input.files().expect("input_ref not bound to file input!");
            let event = event.clone();
            let reader = reader.clone();
            
            if let Some(file) = file_list.get(0) {
                let task = callbacks::read_as_bytes(&File::from(file), move |data| {
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

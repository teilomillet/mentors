// src/components/data_loader.rs

use yew::prelude::*;
use web_sys::{FileReader, HtmlInputElement};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use yew::events::Event;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct Message {
    role: String,
    content: String, // This field will be editable
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct DataEntry {
    messages: Vec<Message>,
}

#[function_component(DataLoader)]
pub fn data_loader() -> Html {
    let data = use_state(|| Vec::new());
    let current_index = use_state(|| 0);
    let link = use_node_ref();

    let onchange = {
        let data = data.clone();
        let current_index = current_index.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let reader = FileReader::new().unwrap_throw();
                    let reader_clone = reader.clone();
                    let data = data.clone();
                    let current_index = current_index.clone();
                    let onloadend_cb = Closure::wrap(Box::new(move |_e: Event| {
                        let array = Uint8Array::new(&reader_clone.result().unwrap_throw());
                        let vec = array.to_vec();
                        let jsonl_str = String::from_utf8_lossy(&vec);
                        let entries: Vec<DataEntry> = jsonl_str
                            .lines()
                            .filter_map(|line| serde_json::from_str(line).ok())
                            .collect();
                        data.set(entries);
                        current_index.set(0);
                    }) as Box<dyn FnMut(_)>);
                    reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                    onloadend_cb.forget();
                    reader.read_as_array_buffer(&file).unwrap_throw();
                }
            }
        })
    };

    // Function to update message content
    let on_edit = {
        let data = data.clone();
        let current_index = current_index.clone();
        move |message_index: usize, new_content: String| {
            let mut data_clone = (*data).clone();
            if let Some(entry) = data_clone.get_mut(*current_index) {
                if let Some(message) = entry.messages.get_mut(message_index) {
                    message.content = new_content;
                }
            }
            data.set(data_clone);
        }
    };

    // Function to save changes
    let save_changes = {
        let data = data.clone();
        move |_| {
            // Serialize the data into a JSONL string
            let serialized_data = data
                .iter()
                .map(|entry| serde_json::to_string(entry).unwrap())
                .collect::<Vec<_>>()
                .join("\n");
    
            // Convert the serialized data to a Uint8Array
            let uint8_array = js_sys::Uint8Array::from(serialized_data.as_bytes());
    
            // Create a js_sys::Array and push the Uint8Array into it
            let array = js_sys::Array::new();
            array.push(&uint8_array);
    
            // Create a new Blob object containing the array
            let mut options = web_sys::BlobPropertyBag::new();
            options.type_("text/plain");
            let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(
                &array,
                &options,
            ).unwrap();
            
            // Create a URL for the blob
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

            // Create a temporary link element for downloading the file
            let document = web_sys::window().unwrap().document().unwrap();
            let body = document.body().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", "modified_data.jsonl").unwrap();

            // Append the link to the body, click it, and then remove it
            body.append_child(&a).unwrap();
            a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
            body.remove_child(&a).unwrap();

            // Clean up the URL object
            web_sys::Url::revoke_object_url(&url).unwrap();

            // Display an alert to notify the user that the changes are being saved
            web_sys::window()
                .unwrap()
                .alert_with_message("Your changes are being saved!")
                .expect("failed to display alert");
        }
    };
    
    html! {
        <>
            {
                if data.len() == 0 {
                    html! {
                        <>
                            <input type="file" accept=".jsonl" {onchange} ref={link}/>
                            <div>{ "Drop a .jsonl file here or click to select one." }</div>
                        </>
                    }
                } else {
                    let current_index_clone_for_previous = current_index.clone();
                    let current_index_clone_for_next = current_index.clone();
    
                    html! {
                        <main>
                            {
                                if let Some(entry) = data.get(*current_index) {
                                    html! {
                                        <div class="messages-container"> // Container for messages
                                            { for entry.messages.iter().enumerate().map(|(index, message)| {
                                                let on_edit = on_edit.clone();
                                                let role_class = if message.role == "user" { "user" } else { "assistant" };
                                                html! {
                                                    <div class="message-container">
                                                        <div class={classes!("message-with-label", role_class)}>
                                                            <textarea
                                                                class="message-editable"
                                                                onchange={Callback::from(move |e: Event| {
                                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                                    on_edit(index, input.value());
                                                                })}
                                                                value={message.content.clone()}
                                                            />
                                                            <div class="message-label">
                                                                { &message.role }
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            })}
                                        </div>
                                    }
                                } else {
                                    html! { <p>{ "No data available or at the end of the dataset." }</p> }
                                }
                            }
                            <div class="button-container">
                                <button
                                    onclick={move |_| {
                                        if *current_index_clone_for_previous > 0 {
                                            current_index_clone_for_previous.set(*current_index_clone_for_previous - 1);
                                        }
                                    }}
                                    disabled={*current_index == 0}
                                >
                                    { "Previous" }
                                </button>
                                <button
                                    onclick={move |_| {
                                        if (*data).get(*current_index_clone_for_next + 1).is_some() {
                                            current_index_clone_for_next.set(*current_index_clone_for_next + 1);
                                        }
                                    }}
                                >
                                    { "Next" }
                                </button>
                                <button onclick={save_changes}>
                                    { "Save Changes" }
                                </button>
                            </div>
                        </main>
                    }
                }
            }
        </>
    }        
}

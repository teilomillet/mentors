use yew::prelude::*;
use serde_json::from_str;
use crate::components::types::DataEntry;

#[function_component(DisplayData)]
pub fn display_data() -> Html {
    let data = use_state(|| Vec::new()); // Example state
    let current_index = use_state(|| 0); // Current message index

    // Load and parse the JSONL data (here we use static data as an example)
    {
        let data = data.clone();
        let current_index = current_index.clone(); // Clone current_index for closure
        use_effect_with_deps(move |_| {
            let jsonl_data = r#"
            {"messages": [{"role": "user", "content": "Message 1"}]}
            {"messages": [{"role": "system", "content": "System message 1"}]}
            {"messages": [{"role": "user", "content": "Message 2"}]}
            {"messages": [{"role": "user", "content": "Message 3"}]}
            {"messages": [{"role": "admin", "content": "Admin message 1"}]}
            "#;
    
            // Simulate fetching and parsing JSONL data
            let entries: Vec<DataEntry> = jsonl_data
                .lines()
                .map(|line| {
                    let entry: DataEntry = from_str(line).ok()?;
                    Some(entry)
                })
                .flatten()
                .collect();
    
            data.set(entries);
    
            // Initialize current_index to 0 when data is loaded
            current_index.set(0);
    
            || ()
        }, ());
    }

    let on_next = {
        let current_index = current_index.clone();
        let data = data.clone();
        Callback::from(move |_| {
            // Increment the index to show the next entry, wrapping around if necessary
            let next_index = (*current_index + 1) % data.len();
            current_index.set(next_index);
        })
    };

    html! {
    <>
        {
            if let Some(entry) = data.get(*current_index) {
                html! {
                    <div>
                        <p>{ &entry.messages[0].role }</p>
                        <p>{ &entry.messages[0].content }</p>
                    </div>
                }
            } else {
                html! { <p>{ "No data available or at the end of the dataset." }</p> }
            }
        }
        <button onclick={on_next}>{ "Next" }</button>
    </>
}
    
}

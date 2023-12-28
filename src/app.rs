// src/app.rs
use yew::prelude::*;
use crate::components::data_loader::DataLoader; // Import the DataLoader component

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <DataLoader /> // Use the DataLoader component
            // You can add more components or HTML elements here as needed
        </>
    }
}

// // src/app.rs
// use yew::prelude::*;

// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <>
//             <h1>{ "Hello, Yew!" }</h1>
//         </>
//     }
// }
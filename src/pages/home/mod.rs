use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="flex flex-col items-center justify-center h-screen w-full">
            <h1 class="p-2 text-4xl text-blue-500 text-center">"Welcome to Leptos!"</h1>
            <button on:click=on_click class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600">
                "Click Me: " {count}
            </button>
        </div>
    }
}

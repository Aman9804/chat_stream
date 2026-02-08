use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let (invitation_code, set_invitation_code) = signal("".to_string());

    let on_submit_code = move |_| {
        // TODO: Implement invitation code validation logic
        leptos::logging::log!("Invitation Code Submitted: {}", invitation_code.get());
    };

    view! {
        <div class="flex flex-col items-center justify-center h-screen w-full bg-gray-50 text-gray-800">
            // Logo Section
            <div class="mb-8 rotate-3 transition-transform hover:rotate-0 duration-300">
                <h1 class="text-6xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600 drop-shadow-sm">
                    "Chat Stream"
                </h1>
            </div>

            // Summary Section
            <div class="max-w-md text-center mb-10 text-lg text-gray-600 leading-relaxed px-4">
                "Experience the future of communication with seamless, real-time messaging. Connect, collaborate, and chat effortlessly."
            </div>

            // Invitation Code Section
            <div class="flex flex-col sm:flex-row gap-3 mb-12 w-full max-w-sm px-4">
                <input
                    type="text"
                    placeholder="Enter Invitation Code"
                    prop:value=invitation_code
                    on:input=move |ev| set_invitation_code.set(event_target_value(&ev))
                    class="flex-1 px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm transition-all"
                />
                <button
                    on:click=on_submit_code
                    class="px-6 py-3 bg-gray-800 text-white font-semibold rounded-xl hover:bg-gray-700 active:scale-95 transition-all shadow-md"
                >
                    "Submit"
                </button>
            </div>

            // Get Started Section
            <a
                href="/chat"
                class="group relative inline-flex items-center justify-center px-8 py-4 font-bold text-white transition-all duration-200 bg-blue-600 font-lg rounded-full hover:bg-blue-700 hover:shadow-lg hover:-translate-y-1 focus:outline-none ring-offset-2 focus:ring-2"
            >
                "Get Started"
                <svg
                    class="w-5 h-5 ml-2 -mr-1 transition-transform group-hover:translate-x-1"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 7l5 5m0 0l-5 5m5-5H6"
                    ></path>
                </svg>
            </a>
        </div>
    }
}

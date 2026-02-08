use crate::db::{get_user, save_user, User};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_query_map};
use uuid::Uuid;

/// Renders the chat page.
#[component]
pub fn ChatPage() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let (is_joined, set_is_joined) = signal(false);
    let (name, set_name) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (phone, set_phone) = signal("".to_string());

    // Derived signal for the room code
    let room_code = Memo::new(move |_| {
        query.with(|q| q.get("code").map(|c| c.to_string()).unwrap_or_default())
    });

    // Load user on mount (LocalResource because Rexie is !Send)
    let user_resource = LocalResource::new(move || async move { get_user().await.ok().flatten() });

    Effect::new(move |_| {
        if let Some(Some(user)) = user_resource.get() {
            set_name.set(user.name);
            set_email.set(user.email);
            set_phone.set(user.phone);
        }
    });

    let on_submit = move |_| {
        let user = User {
            name: name.get(),
            email: email.get(),
            phone: phone.get(),
            created_at: js_sys::Date::now() as u64,
        };

        let navigate = navigate.clone();

        spawn_local(async move {
            match save_user(user).await {
                Ok(_) => leptos::logging::log!("User saved successfully"),
                Err(e) => leptos::logging::error!("Failed to save user: {:?}", e),
            }

            let current_code = room_code.get();
            if current_code.is_empty() {
                let new_code = Uuid::new_v4().to_string();
                navigate(&format!("/chat?code={}", new_code), Default::default());
            }
            set_is_joined.set(true);
        });
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen w-full bg-gray-50 p-4">
            {move || if !is_joined.get() {
                view! {
                    <div class="bg-white p-8 rounded-2xl shadow-xl w-full max-w-md transition-all">
                        <h2 class="text-3xl font-bold text-center mb-6 text-gray-800">
                            {move || if room_code.get().is_empty() { "Create New Chat" } else { "Join Chat" }}
                        </h2>

                        <div class="space-y-4">
                            <div class="flex flex-col gap-1">
                                <label class="text-sm font-medium text-gray-600">"Full Name"</label>
                                <input
                                    type="text"
                                    placeholder="John Doe"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
                                />
                            </div>

                            <div class="flex flex-col gap-1">
                                <label class="text-sm font-medium text-gray-600">"Email Address"</label>
                                <input
                                    type="email"
                                    placeholder="john@example.com"
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
                                />
                            </div>

                            <div class="flex flex-col gap-1">
                                <label class="text-sm font-medium text-gray-600">"Phone Number"</label>
                                <input
                                    type="tel"
                                    placeholder="+1 (555) 000-0000"
                                    prop:value=phone
                                    on:input=move |ev| set_phone.set(event_target_value(&ev))
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
                                />
                            </div>

                            <button
                                on:click=on_submit.clone()
                                class="w-full mt-6 py-4 bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold rounded-xl shadow-lg hover:shadow-xl hover:-translate-y-0.5 transition-all active:scale-95"
                            >
                                {move || if room_code.get().is_empty() { "Create & Invite" } else { "Join Chat" }}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="flex flex-col h-screen w-full max-w-6xl bg-white shadow-2xl rounded-2xl overflow-hidden my-4 border border-gray-200">
                        // Header
                        <div class="bg-white border-b border-gray-100 p-4 flex justify-between items-center bg-gray-50/50">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-full bg-gradient-to-tr from-blue-500 to-purple-500 flex items-center justify-center text-white font-bold text-lg shadow-md">
                                    {move || name.get().chars().next().unwrap_or('?').to_uppercase().to_string()}
                                </div>
                                <div>
                                    <h3 class="font-bold text-gray-800">{move || name.get()}</h3>
                                    <p class="text-xs text-green-500 font-medium flex items-center gap-1">
                                        <span class="w-2 h-2 rounded-full bg-green-500"></span>
                                        "Online"
                                    </p>
                                </div>
                            </div>

                            <div class="flex items-center gap-2 bg-blue-50 px-4 py-2 rounded-lg border border-blue-100">
                                <span class="text-xs font-bold text-blue-600 uppercase tracking-wider">"Invite Code"</span>
                                <code class="font-mono font-bold text-gray-800 text-lg">{move || room_code.get()}</code>
                                <button
                                    class="p-1 hover:bg-blue-100 rounded-md transition-colors text-blue-600"
                                    title="Copy Code"
                                    on:click=move |_| {
                                        // TODO: Implement copy to clipboard
                                        leptos::logging::log!("Copy code: {}", room_code.get());
                                    }
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 012 2v6a2 2 0 01-2 2h-8a2 2 0 01-2-2v-6a2 2 0 012-2z"></path></svg>
                                </button>
                            </div>
                        </div>

                        // Chat Area (Placeholder)
                        <div class="flex-1 bg-gray-50 p-6 overflow-y-auto flex flex-col items-center justify-center text-gray-400">
                            <div class="w-24 h-24 bg-gray-200 rounded-full mb-4 flex items-center justify-center">
                                <svg class="w-10 h-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path></svg>
                            </div>
                            <p class="text-lg font-medium">"Welcome to the chat!"</p>
                            <p class="text-sm">"Share the invitation code to start messaging."</p>
                        </div>

                        // Input Area (Placeholder)
                        <div class="p-4 bg-white border-t border-gray-100">
                             <div class="flex gap-2">
                                <input
                                    type="text"
                                    placeholder="Type a message..."
                                    class="flex-1 px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                                />
                                <button class="px-6 py-3 bg-blue-600 text-white font-bold rounded-xl hover:bg-blue-700 transition-all shadow-md active:scale-95 flex items-center gap-2">
                                    <span>"Send"</span>
                                    <svg class="w-4 h-4 transform rotate-90" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
                                </button>
                             </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

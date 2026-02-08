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
        <div class="flex flex-col items-center justify-center min-h-screen w-full bg-gray-50 p-4 md:p-6 transition-all duration-300">
            {move || if !is_joined.get() {
                view! {
                    <div class="bg-white p-8 rounded-2xl shadow-2xl w-full max-w-md transition-all border border-gray-100 hover:shadow-cyan-500/10">
                        <h2 class="text-3xl font-extrabold text-center mb-6 text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600">
                            {move || if room_code.get().is_empty() { "Create New Chat" } else { "Join Chat" }}
                        </h2>

                        <div class="space-y-5">
                            <div class="flex flex-col gap-2">
                                <label class="text-sm font-semibold text-gray-700 ml-1">"Full Name"</label>
                                <input
                                    type="text"
                                    placeholder="John Doe"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    class="w-full px-5 py-4 bg-gray-50 border border-gray-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all shadow-sm"
                                />
                            </div>

                            <div class="flex flex-col gap-2">
                                <label class="text-sm font-semibold text-gray-700 ml-1">"Email Address"</label>
                                <input
                                    type="email"
                                    placeholder="john@example.com"
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    class="w-full px-5 py-4 bg-gray-50 border border-gray-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all shadow-sm"
                                />
                            </div>

                            <div class="flex flex-col gap-2">
                                <label class="text-sm font-semibold text-gray-700 ml-1">"Phone Number"</label>
                                <input
                                    type="tel"
                                    placeholder="+1 (555) 000-0000"
                                    prop:value=phone
                                    on:input=move |ev| set_phone.set(event_target_value(&ev))
                                    class="w-full px-5 py-4 bg-gray-50 border border-gray-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all shadow-sm"
                                />
                            </div>

                            <button
                                on:click=on_submit.clone()
                                class="w-full mt-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-xl hover:-translate-y-0.5 transition-all active:scale-95 duration-200"
                            >
                                {move || if room_code.get().is_empty() { "Create & Invite" } else { "Join Chat" }}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="flex flex-col w-full max-w-6xl my-auto h-[85vh] md:h-[90vh] gap-4">
                        // Header
                        <div class="bg-white/90 backdrop-blur-md p-4 md:p-6 flex justify-between items-center rounded-2xl shadow-sm border border-white/20">
                            <div class="flex items-center gap-4">
                                <div class="w-12 h-12 rounded-full bg-gradient-to-tr from-blue-500 to-purple-500 flex items-center justify-center text-white font-bold text-xl shadow-md ring-2 ring-white">
                                    {move || name.get().chars().next().unwrap_or('?').to_uppercase().to_string()}
                                </div>
                                <div>
                                    <h3 class="font-bold text-gray-800 text-lg">{move || name.get()}</h3>
                                    <p class="text-xs text-green-500 font-bold flex items-center gap-1.5 uppercase tracking-wide">
                                        <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
                                        "Online"
                                    </p>
                                </div>
                            </div>

                            <div class="flex items-center gap-3 bg-gray-50 px-5 py-2.5 rounded-xl border border-gray-200 shadow-sm transition-colors hover:border-blue-200 hover:bg-blue-50/50">
                                <span class="hidden md:inline text-xs font-bold text-blue-600 uppercase tracking-wider">"Invite Code"</span>
                                <code class="font-mono font-bold text-gray-800 text-base md:text-lg">{move || room_code.get()}</code>
                                <button
                                    class="p-2 hover:bg-white rounded-lg transition-all text-gray-400 hover:text-blue-600 hover:shadow-sm active:scale-95"
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
                        <div class="flex-1 bg-white/60 backdrop-blur-sm p-8 overflow-y-auto flex flex-col items-center justify-center text-gray-400 rounded-2xl shadow-sm border border-white/20">
                            <div class="w-32 h-32 bg-gray-100 rounded-full mb-6 flex items-center justify-center shadow-inner">
                                <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path></svg>
                            </div>
                            <h3 class="text-xl font-bold text-gray-300 mb-2">"Welcome to the chat!"</h3>
                            <p class="text-sm font-medium text-gray-400">"Share the invitation code above to start messaging."</p>
                        </div>

                        // Input Area (Placeholder)
                        <div class="p-4 bg-white rounded-2xl shadow-sm border border-white/20">
                             <div class="flex gap-3">
                                <input
                                    type="text"
                                    placeholder="Type a message..."
                                    class="flex-1 px-6 py-4 bg-gray-50 border border-gray-200 rounded-2xl focus:outline-none focus:bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all shadow-sm font-medium"
                                />
                                <button class="px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold rounded-2xl hover:shadow-lg hover:-translate-y-0.5 transition-all active:scale-95 flex items-center gap-2 group">
                                    <span>"Send"</span>
                                    <svg class="w-5 h-5 transform rotate-90 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
                                </button>
                             </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

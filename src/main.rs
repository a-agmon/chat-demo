mod llm_utils;

use leptos::{logging::log, prelude::*, task::spawn_local};
use llm_utils::LLMRequest;
use reqwest::Client;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

async fn send_message(message: String) -> String {
    log!("Sending message: {}", message);
    let client = Client::new();
    let response = client
        .post("http://localhost:3000/generate")
        .header("Content-Type", "text/plain")
        .body(message)
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MessageSource {
    User,
    Bot,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Message {
    text: String,
    sender: MessageSource,
}

#[component]
fn App() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ArcRwSignal<Message>>::new());
    set_messages.write().push(ArcRwSignal::new(Message {
        text: "Welcome! How can I help you?".to_string(),
        sender: MessageSource::Bot,
    }));

    view! {
        <div class="max-w-2xl mx-auto bg-white rounded-lg shadow-md border border-gray-350">
            <Header/>
             <div class="px-4 py-4 h-96 overflow-y-scroll">
                <ChatView messages=messages/>
            </div>
            <Footer messages=set_messages/>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
         <div class="flex items-center justify-start px-4 py-2 border-b">
         <div>
                 <img src="https://1000logos.net/wp-content/uploads/2023/11/Copilot-Logo-500x281.png" alt="Logo" class="h-8 w-15" />
             </div>
             <div class="flex items-center">
                 <span class="text-sm font-semibold text-gray-700">Database Agent</span>
                 <div class="w-px h-4 mx-2 bg-gray-400"></div>
                 <span class="text-sm text-gray-500">Retail</span>
             </div>
         </div>
     }
}

#[component]
fn ChatView(messages: ReadSignal<Vec<ArcRwSignal<Message>>>) -> impl IntoView {
    view! {
        <For
            each=move || messages.get()
            key=|state| state.clone()
            let:child
        >
            <UserMessage message=RwSignal::from(child)/>
        </For>
    }
}

#[component]
fn UserMessage(message: RwSignal<Message>) -> impl IntoView {
    let message = message.read_only();
    let message_text = message.get().text;
    let is_bot = message.get().sender == MessageSource::Bot;
    view! {
        <div class="flex items-start mb-6 max-w-full">
            <div class="w-8 h-8 bg-gradient-to-r from-indigo-500 to-blue-500 rounded-full flex-shrink-0 text-white flex items-center justify-center">
                <i class=move || if is_bot { "fas fa-robot" } else { "fas fa-user" }></i>
            </div>
            <div class="ml-3 flex-1 min-w-0">
                <p class="text-sm font-medium text-gray-700 mb-1">{move || if is_bot { "Copilot" } else { "You" }}</p>
                <p class="text-sm text-gray-600 break-words whitespace-normal overflow-hidden text-ellipsis" inner_html={
                    markdown::to_html(&message_text.trim_matches('"').replace("\\n", "\n"))
                }/>
            </div>
        </div>
    }
}

#[component]
fn Footer(messages: WriteSignal<Vec<ArcRwSignal<Message>>>) -> impl IntoView {
    let (user_input, set_user_input) = signal("".to_string());

    view! {
        <div class="px-4 py-3 bg-gray-50 border-t flex items-center space-x-3">
            <input type="text" placeholder="Whats on your mind?"
            class="flex-grow text-sm bg-gray-100 border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
            on:input:target=move |e| set_user_input.set(e.target().value())
            prop:value=user_input/>

            <button class="text-blue-500" on:click=move |_| {
                messages.update(|messages| {
                    messages.push(ArcRwSignal::new(Message {
                        text: user_input.get(),
                        sender: MessageSource::User,
                    }));
                });
                let input = user_input.get();
                set_user_input.set(String::new());
                spawn_local(async move {
                    let response = send_message(input).await;
                    messages.write().push(ArcRwSignal::new(Message {
                        text: response,
                        sender: MessageSource::Bot,
                    }));
                });
            }>
                <i class="fas fa-paper-plane"></i>
            </button>
        </div>

    }
}

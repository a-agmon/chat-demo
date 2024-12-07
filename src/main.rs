use leptos::{attr::target, logging::log, prelude::*, task::spawn_local};
use leptos_use::whenever;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

use gloo_timers::future::TimeoutFuture;
// this function sends the message to the backend and then adds it to the messages list when its returned
async fn send_message(message: String) -> String {
    // TimeoutFuture::new(1_000).await;
    log!("Sending message: {}", message);
    String::from("Here is the response")
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Message {
    text: String,
    sender: String,
}

#[component]
fn App() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    set_messages.write().push(Message {
        text: "Hello".to_string(),
        sender: "me".to_string(),
    });
    set_messages.write().push(Message {
        text: "World".to_string(),
        sender: "me".to_string(),
    });

    view! {
        <div class="max-w-md mx-auto bg-white rounded-lg shadow-md border border-gray-350">
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
        <div class="flex items-center justify-between px-4 py-2 border-b">
            <div class="flex items-center">
                <span class="text-sm font-semibold text-gray-700">Soemthing</span>
                <div class="w-px h-4 mx-2 bg-gray-400"></div>
                <span class="text-sm text-gray-500">Soemthing</span>
            </div>
            <div>
                <i class="fas fa-shield-alt text-green-500"></i>
            </div>
        </div>
    }
}

#[component]
fn ChatView(messages: ReadSignal<Vec<Message>>) -> impl IntoView {

    view! {
        <p>{move || messages.get().len()}</p>
        <For
            each=move || messages.get()
            key=|state| state.text.clone()
            let:child
        >
            <UserMessage message=child.text/>
        </For>
    }
}

#[component]
fn UserMessage(message: String) -> impl IntoView {
    view! {
        <div class="flex items-start mb-4">
            <div class="w-8 h-8 bg-blue-500 rounded-full flex-shrink-0 text-white flex items-center justify-center">
                <i class="fas fa-user"></i>
            </div>
            <div class="ml-3">
                <p class="text-sm font-medium text-gray-700">You</p>
                <p class="text-sm text-gray-600">{message}</p>
            </div>
        </div>
    }
}

#[component]
fn Footer(messages: WriteSignal<Vec<Message>>) -> impl IntoView {
    let (user_input, set_user_input) = signal("".to_string());

    view! {
        <div class="px-4 py-3 bg-gray-50 border-t flex items-center space-x-3">
            <input type="text" placeholder="Whats on your mind?"
            class="flex-grow text-sm bg-gray-100 border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
            on:input:target=move |e| set_user_input.set(e.target().value())
            prop:value=user_input/>

            <button class="text-blue-500" on:click=move |_| {
                messages.write().push(Message {
                    text: user_input.get(),
                    sender: "me".to_string(),
                });
                let input = user_input.get();
                set_user_input.set(String::new());
                spawn_local(async move {
                    let response = send_message(input).await;
                    messages.write().push(Message {
                        text: response,
                        sender: "bot".to_string(),
                    });
                });
            }>
                <i class="fas fa-paper-plane"></i>
            </button>
        </div>
    }
}

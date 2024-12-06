use leptos::{attr::target, logging::log, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(MainView)
}

#[component]
fn MainView() -> impl IntoView {
    let (messages, set_messages) = signal(Messages::new());
    set_messages.update(|collection| collection.push("yup".to_string()));
    set_messages.write().push("Hello".to_string());
    set_messages.write().push("World".to_string());

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

type Messages = Vec<String>;
#[component]
fn ChatView(messages: ReadSignal<Messages>) -> impl IntoView {
    view! {
        <For
            each=move || messages.get()
            key=|state| state.clone()
            let:child
        >
            <UserMessage message=child/>
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
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            >
            <b> Click me: </b>
            {count} //
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
        <List/>

    }
}

#[component]
fn List() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        // this will just render "012"
        <br/>
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>
    }
}

#[component]
fn Footer(messages: WriteSignal<Messages>) -> impl IntoView {
    let (user_input, set_user_input) = signal("".to_string());
    view! {
        <div class="px-4 py-3 bg-gray-50 border-t flex items-center space-x-3">
            <input type="text" placeholder="Whats on your mind?" class="flex-grow text-sm bg-gray-100 border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
            on:input:target=move |e| set_user_input.set(e.target().value())
            prop:value=user_input/>

            <button class="text-blue-500" on:click=move |_| {
                let input = user_input.get();
                log!("Sending message: {}",input);
                messages.write().push(input);
            }>
                <i class="fas fa-paper-plane"></i>
            </button>
        </div>
    }
}

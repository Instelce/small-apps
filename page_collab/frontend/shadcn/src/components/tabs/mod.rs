use leptos::logging::log;
use leptos::*;

pub trait TabValue {
    fn set(value: &str) -> Self;
    fn get(&self) -> String;
}

#[component]
pub fn Tabs(
    value: impl TabValue,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let (value, set_value) = create_signal(value.get());
    provide_context(set_value);
    provide_context(value);

    view! { class=class, <div>{children()}</div> }
}

#[component]
pub fn TabsList(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { class=class,
        <div class="bg-muted text-muted-foreground inline-flex h-10 items-center justify-center rounded-md p-1">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(value: impl TabValue + Clone + 'static, children: Children) -> impl IntoView {
    // Retrieve tab value signals
    let set_value = use_context::<WriteSignal<String>>().expect("no `set_value` signal provided");
    let tab_value = use_context::<ReadSignal<String>>().expect("no `value` signal provided");

    // Clone for the update_state closure
    let value_state = value.clone().get();

    // Update closure
    let update_tab_value = move |_| set_value.update(|v| *v = value.get());
    let update_state = move || match value_state == tab_value() {
        true => "active",
        false => "",
    };

    view! {
        <button
            class="ring-offset-background focus-visible:ring-ring data-[state=active]:bg-background data-[state=active]:text-foreground inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:shadow-sm"
            on:click=update_tab_value
            attr:data-state=update_state
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(value: impl TabValue + 'static, children: Children) -> impl IntoView {
    let tab_value = use_context::<ReadSignal<String>>().expect("no `value` signal provided");

    let update_visibility = move || tab_value() != value.get();

    view! {
        <div
            class="ring-offset-background focus-visible:ring-ring mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2"
            class:hidden=update_visibility
        >
            {children()}
        </div>
    }
}

use leptos::*;

#[component]
pub fn Label(
    #[prop(optional)] for_input: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <label
            for=for_input
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                .to_owned() + class
        >
            {children()}
        </label>
    }
}

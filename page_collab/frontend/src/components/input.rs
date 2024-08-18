use leptos::*;

#[component]
pub fn Input(
    #[prop(optional)] itype: &'static str,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] name: &'static str,
    #[prop(optional)] maxlength: &'static str,
    #[prop(optional)] minlength: &'static str,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <input
            type=itype
            id=id
            name=name
            maxlength=maxlength
            minlength=minlength
            placeholder=placeholder
            class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                .to_owned() + class
        />
    }
}

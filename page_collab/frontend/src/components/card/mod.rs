use leptos::*;


#[component]
pub fn Card(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground rounded-lg border shadow-sm".to_owned()
            + class>{children()}</div>
    }
}

#[component]
pub fn CardHeader(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="flex flex-col space-y-1.5 p-6".to_owned() + class>{children()}</div> }
}

#[component]
pub fn CardTitle(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="text-lg font-semibold leading-none tracking-tight".to_owned()
            + class>{children()}</div>
    }
}

#[component]
pub fn CardDescription(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="text-muted-foreground text-sm".to_owned() + class>{children()}</div> }
}

#[component]
pub fn CardContent(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="p-6 pt-0".to_owned() + class>{children()}</div> }
}

#[component]
pub fn CardFooter(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="flex items-center p-6 pt-0".to_owned() + class>{children()}</div> }
}
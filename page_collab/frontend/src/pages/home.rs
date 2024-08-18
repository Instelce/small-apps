use crate::components::{
    button::{BVariant, Button},
    card::*,
    input::Input,
    label::Label,
    select::*,
    tabs::*,
};
use leptos::logging::log;
use leptos::*;

#[derive(Debug, Clone)]
struct FormsTabValue(String);

impl TabValue for FormsTabValue {
    fn set(value: &str) -> Self {
        Self(value.to_string())
    }

    fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone)]
struct Framework {
    value: String,
    label: String,
}

impl Framework {
    fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (data, _) = create_signal(vec![
        Framework::new("sveltekit", "SvelteKit"),
        Framework::new("next", "Next.js"),
        Framework::new("astro", "Astro"),
        Framework::new("nust", "Nust.js"),
    ]);

    view! {
        <Button variant=BVariant::Ghost>"coucou aze"</Button>

        <Button on:click=move |_| {
            log!("coucou");
        }>"coucou aze"</Button>

        <Tabs value=FormsTabValue::set("card") class="w-[400px]">
            <TabsList class="w-full grid grid-cols-3">
                <TabsTrigger value=FormsTabValue::set("register")>"Register"</TabsTrigger>
                <TabsTrigger value=FormsTabValue::set("login")>"Login"</TabsTrigger>
                <TabsTrigger value=FormsTabValue::set("card")>"Card"</TabsTrigger>
            </TabsList>
            <TabsContent value=FormsTabValue::set("register")>""</TabsContent>
            <TabsContent value=FormsTabValue::set("login")>"Login"</TabsContent>
            <TabsContent value=FormsTabValue::set("card")>
                ""
            </TabsContent>
        </Tabs>
    }
}

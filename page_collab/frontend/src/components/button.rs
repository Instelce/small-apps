use std::collections::HashMap;

use leptos::*;

#[derive(Default, PartialEq, Eq, Hash)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

pub type BVariant = ButtonVariant;

#[derive(Default, PartialEq, Eq, Hash)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

pub type BSize = ButtonSize;

struct Style {
    base: String,
    variant: HashMap<ButtonVariant, String>,
    size: HashMap<ButtonSize, String>,
}

impl Style {
    pub fn init() -> Self {
        Self {
            base: "ring-offset-background focus-visible:ring-ring inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".into(),
            variant: HashMap::from([
                (ButtonVariant::Default, "bg-primary text-primary-foreground hover:bg-primary/90".into()),
                (ButtonVariant::Destructive, "bg-destructive text-destructive-foreground hover:bg-destructive/90".into()),
                (ButtonVariant::Outline, "border-input bg-background hover:bg-accent hover:text-accent-foreground border".into()),
                (ButtonVariant::Secondary, "bg-secondary text-secondary-foreground hover:bg-secondary/80".into()),
                (ButtonVariant::Ghost, "hover:bg-accent hover:text-accent-foreground".into()),
                (ButtonVariant::Link, "text-primary underline-offset-4 hover:underline".into()),
            ]),
            size: HashMap::from([
                (ButtonSize::Default, "h-10 px-4 py-2".into()),
                (ButtonSize::Sm, "h-9 rounded-md px-3".into()),
                (ButtonSize::Lg, "h-11 rounded-md px-8".into()),
                (ButtonSize::Icon, "h-10 w-10".into()),
            ])
        }
    }

    pub fn variants(&self, variant: Option<ButtonVariant>, size: Option<ButtonSize>) -> String {
        let variant_key = variant.unwrap_or_default();
        let size_key = size.unwrap_or_default();
        format!(
            "{} {} {}",
            self.base, self.variant[&variant_key], self.size[&size_key]
        )
    }
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] _type: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let style = Style::init();

    view! {
        <button type=_type class=style.variants(variant, size) + " " + class>
            {children()}
        </button>
    }
}

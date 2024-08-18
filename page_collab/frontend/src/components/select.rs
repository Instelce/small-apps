use html::{Button, Div};
use leptos::*;
use leptos_lucide::{icons::*, LucideAttributes, LucideAttributesCtx};
use leptos_use::*;
use styled::style;
use tiny_id::ShortCodeGenerator;

// Style
static SELECT_TRIGGER_CLASSES: &str = "";

// Context

#[derive(Default, Clone)]
struct SelectContext {
    // use in SelectContent to get the SelectTrigger
    pub id: String,
    pub value: String,
    pub label: String,
    pub open: bool,
    pub selected: bool,
    pub trigger: NodeRef<Button>,
}

fn use_select_context() -> (ReadSignal<SelectContext>, WriteSignal<SelectContext>) {
    (
        use_context::<ReadSignal<SelectContext>>()
            .expect("Place `Select*` element inside Select element"),
        use_context::<WriteSignal<SelectContext>>()
            .expect("Place `Select*` element inside Select element"),
    )
}

#[component]
pub fn Select(
    #[prop(optional)] id: &'static str,
    #[prop(optional)] name: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    // Provide context
    let (context, set_context) = create_signal(SelectContext::default());
    provide_context(context);
    provide_context(set_context);

    let udpate_value = move || context().value;

    // Set id
    let mut gen = ShortCodeGenerator::new_uppercase(6);
    let select_id = gen.next_string();
    set_context.update(|v| v.id = select_id.clone());

    view! { class=class,
        <div>
            {children()}
            <input
                id=id
                name=name
                type="hidden"
                attr:aria-hidden=true
                hidden=true
                value=udpate_value
            />
        </div>
    }
}

#[component]
pub fn SelectTrigger(
    #[prop(optional)] id: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    // Ref
    let button_ref = create_node_ref::<Button>();

    // Add icon classes
    let attributes = LucideAttributes::new().set_classes("h-4 w-4 opacity-50");
    let attributes_ctx = LucideAttributesCtx(RwSignal::new(attributes));
    provide_context(attributes_ctx);

    // Retrieve context
    let (context, set_context) = use_select_context();

    let update_open = move |_| {
        set_context.update(|v| {
            v.open = !v.open;
            v.trigger = button_ref;
        });
        // logging::log!("{:?}", context());
    };

    let update_icon = move || match context().open {
        true => view! { <ChevronUp /> },
        false => view! { <ChevronDown /> },
    };

    view! {
        <button
            node_ref=button_ref
            attr:data-id=move || context().id
            id=id
            role="combobox"
            type="button"
            class="border-input bg-background ring-offset-background focus-visible:ring-ring aria-[invalid]:border-destructive data-[placeholder]:[&>span]:text-muted-foreground flex h-10 w-full items-center justify-between rounded-md border px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1"
                .to_owned() + class
            on:click=update_open
        >
            {children()}
            {update_icon}
        </button>
    }
}

#[component]
pub fn SelectValue(
    placeholder: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let (context, _) = use_select_context();

    let update_class = move || match context().selected {
        true => "",
        false => "text-muted-foreground",
    };
    let update_content = move || match context().selected {
        true => context().label,
        false => placeholder.to_string(),
    };

    view! { <span class=update_class>{update_content}</span> }
}

#[derive(Default, Clone, Debug)]
struct ContentPosition {
    pub width: i32,
    pub top: i32,
    pub left: i32,
}

#[component]
pub fn SelectContent(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    // Retrieve context
    let (context, set_context) = use_select_context();

    // Close when clicking outside the content
    let content_ref = create_node_ref::<Div>();
    let _ = on_click_outside(content_ref, move |_| set_context.update(|v| v.open = false));

    let update_visibility = move || !context().open;

    let (bounding, set_bounding) = create_signal(ContentPosition::default());

    // remove warning
    create_effect(move |_| {
        if context().open && bounding().width == 0 {
            let UseElementBoundingReturn {
                top,
                left,
                width,
                height,
                ..
            } = use_element_bounding(context().trigger.clone());

            set_bounding.set(ContentPosition {
                width: width.get() as i32,
                top: (top.get() + height.get()) as i32 + 5,
                left: left.get() as i32,
            });
        }
    });

    view! {
        <div
            node_ref=content_ref
            role="listbox"
            class="bg-popover text-popover-foreground z-50 min-w-[8rem] overflow-hidden rounded-md border shadow-md outline-none absolute"
                .to_owned() + " " + class
            style:width=move || format!("{}px", bounding().width)
            style:top=move || format!("{}px", bounding().top)
            style:left=move || format!("{}px", bounding().left)
            class:hidden=update_visibility
        >
            <div class="w-full p-1">{children()}</div>
        </div>
    }
}

#[component]
pub fn SelectGroup(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { class=class, <div role="group">{children()}</div> }
}

#[component]
pub fn SelectLabel(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="py-1.5 pl-8 pr-2 text-sm font-semibold".to_owned() + class>{children()}</div> }
}

#[component]
pub fn SelectItem(
    value: String,
    label: String,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let (context, set_context) = use_select_context();

    let label_cloned = label.clone();
    let value_cloned = value.clone();
    let udpate_value = move |_| {
        set_context.update(|v| {
            v.value = value_cloned.clone();
            v.label = label_cloned.clone();
            v.selected = true;
            v.open = false;
        });
    };

    let update_icon_visibility = move || match context().value == value {
        true => view! {
            <span>
                <Check />
            </span>
        },
        false => view! { <span></span> },
    };

    view! {
        <div
            role="option"
            class="hover:bg-accent hover:text-accent-foreground relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
                .to_owned() + class
            on:click=udpate_value
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                {update_icon_visibility}
            </span>
            {label}
        </div>
    }
}

#[component]
pub fn SelectSeparator(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class="bg-muted -mx-1 my-1 h-px".to_owned() + class>{children()}</div> }
}

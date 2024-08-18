use leptos::*;
use shadcn::*;

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

pub fn CardStorie() -> impl IntoView {
    let (data, _) = create_signal(vec![
        Framework::new("sveltekit", "SvelteKit"),
        Framework::new("next", "Next.js"),
        Framework::new("astro", "Astro"),
        Framework::new("nust", "Nust.js"),
    ]);

    view! {
        <Card class="w-full">
            <CardHeader>
                <CardTitle>Create project</CardTitle>
                <CardDescription>Deploy your new project in one-click.</CardDescription>
            </CardHeader>
            <CardContent>
                <form>
                    <div class="grid w-full items-center gap-4">
                        <div class="flex flex-col space-y-1.5">
                            <Label for_input="name">Name</Label>
                            <Input id="name" attr:placeholder="Name of your project" />
                        </div>
                        <div class="flex flex-col space-y-1.5">
                            <Label for_input="framework">Framework</Label>
                            <Select>
                                <SelectTrigger id="framework">
                                    <SelectValue placeholder="Select" />
                                </SelectTrigger>
                                <SelectContent>
                                    <For each=data key=|state| state.value.clone() let:framework>
                                        <SelectItem value=framework.value label=framework.label />
                                    </For>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex justify-between">
                <Button variant=BVariant::Outline>"Cancel"</Button>
                <Button>"Deploy"</Button>
            </CardFooter>
        </Card>
    }
}

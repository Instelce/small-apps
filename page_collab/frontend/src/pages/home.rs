use ev::SubmitEvent;
use leptos::*;
use leptos_router::Form;
use leptos_router::FromFormData;
use shadcn::components::{
    button::{BVariant, Button},
    card::*,
    input::Input,
    label::Label,
    select::*,
    tabs::*,
};
use shared::params::user::LoginParams;

use crate::api::{use_api, Api};

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

#[component]
pub fn Home() -> impl IntoView {
    let api = use_api();
    view! {
        <div class="w-full h-[100vh] grid place-items-center">
            <Tabs value=FormsTabValue::set("register") class="w-[400px]">
                <TabsList class="w-full grid grid-cols-2">
                    <TabsTrigger value=FormsTabValue::set("register")>"Register"</TabsTrigger>
                    <TabsTrigger value=FormsTabValue::set("login")>"Login"</TabsTrigger>
                </TabsList>
                <TabsContent value=FormsTabValue::set("register")>
                    <Card class="w-full">
                        <CardHeader>
                            <CardTitle>"Welcome"</CardTitle>
                            <CardDescription>"Create your account here."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form>
                                <div class="grid w-full items-center gap-4">
                                    <div class="flex flex-col space-y-1.5">
                                        <Label for_input="name">Name</Label>
                                        <Input id="name" />
                                    </div>
                                    <div class="flex flex-col space-y-1.5">
                                        <Label for_input="email">"Email"</Label>
                                        <Input id="email" />
                                    </div>
                                    <div class="flex flex-col space-y-1.5">
                                        <Label for_input="password">"Password"</Label>
                                        <Input id="password" _type="password" />
                                    </div>
                                </div>

                                <Button _type="submit" class="w-full mt-4">
                                    "Create an account"
                                </Button>
                            </form>
                        </CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value=FormsTabValue::set("login")>
                    <Card class="w-full">
                        <CardHeader>
                            <CardTitle>"Welcome back"</CardTitle>
                            <CardDescription>"Happy to see you again !"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form method="post" on:submit=move |e: SubmitEvent| {
                                let data = LoginParams::from_event(&e);

                                match data {
                                    Ok(data) => {
                                        if data.email == "nope!" {
                                            e.prevent_default();
                                        } else {
                                            e.prevent_default();
                                            let query = api.get().login(data);
                                            let response = query.data.get().unwrap();

                                            match response {
                                                Ok(data) => {
                                                    logging::log!("{data:?}");
                                                },
                                                Err(e) => {
                                                    logging::log!("{e}");
                                                }
                                            }

                                        }
                                    }
                                    Err(err) => {logging::log!("{err}"); e.prevent_default();},
                                };
                            }>
                                <div class="grid w-full items-center gap-4">
                                    <div class="flex flex-col space-y-1.5">
                                        <Label for_input="email">"Email"</Label>
                                        <Input id="email" name="email" />
                                    </div>
                                    <div class="flex flex-col space-y-1.5">
                                        <Label for_input="password">"Password"</Label>
                                        <Input id="password" name="password" _type="password" />
                                    </div>
                                </div>

                                <Button _type="submit" class="w-full mt-4">
                                    "Log in"
                                </Button>
                            </form>
                        </CardContent>
                    </Card>
                </TabsContent>
            </Tabs>
        </div>
    }
}

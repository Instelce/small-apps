use leptos::*;

pub fn CardStorie() -> impl IntoView {
    view! {
                // <Card class="w-full">
                //     <CardHeader>
                //         <CardTitle>Create project</CardTitle>
                //         <CardDescription>Deploy your new project in one-click.</CardDescription>
                //     </CardHeader>
                //     <CardContent>
                //         <form>
                //             <div class="grid w-full items-center gap-4">
                //                 <div class="flex flex-col space-y-1.5">
                //                     <Label for_input="name">Name</Label>
                //                     <Input id="name" attr:placeholder="Name of your project" />
                //                 </div>
                //                 <div class="flex flex-col space-y-1.5">
                //                     <Label for_input="framework">Framework</Label>
                //                     <Select>
                //                         <SelectTrigger id="framework">
                //                             <SelectValue placeholder="Select" />
                //                         </SelectTrigger>
                //                         <SelectContent>
                //                             <For
                //                                 each=data
                //                                 key=|state| state.value.clone()
                //                                 let:framework
                //                             >
                //                                 <SelectItem value=framework.value label=framework.label />
                //                             </For>
                //                         </SelectContent>
                //                     </Select>
                //                 </div>
                //             </div>
                //         </form>
                //     </CardContent>
                //     <CardFooter class="flex justify-between">
                //         <Button variant=BVariant::Outline>"Cancel"</Button>
                //         <Button>"Deploy"</Button>
                //     </CardFooter>
                // </Card>
    }
}

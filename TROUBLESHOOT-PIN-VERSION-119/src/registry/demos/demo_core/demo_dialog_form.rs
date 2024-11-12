use leptos::*;

use crate::registry::ui::{
    dialog::{
        DialogBody, DialogButtonFormCancel, DialogButtonFormSubmit, DialogComponent, DialogContent,
        DialogFooter, DialogForm, DialogProvider, DialogTitle, DialogTrigger,
    },
    form::FormField,
    input::Input,
};

#[component]
pub fn DemoDialogForm() -> impl IntoView {
    view! {
        <DialogProvider>

            <DialogTrigger>"Change payment method"</DialogTrigger>

            <DialogComponent>
                <DialogContent>
                    <DialogTitle>"Change payment method"</DialogTitle>
                    <DialogForm>
                        <DialogBody class="flex flex-col gap-2">
                            <FormField>
                                <label class="w-1/3 mr-auto text-gray-400">"Card number"</label>
                                <Input name="card-number" />
                            </FormField>

                            <FormField>
                                <label class="w-1/3 mr-auto text-gray-400">"Expiration"</label>
                                <Input name="expiration" />
                            </FormField>

                            <FormField>
                                <label class="w-1/3 mr-auto text-gray-400">"CVC"</label>
                                <Input r#type="password" name="cvc" placeholder="•••" />
                            </FormField>
                        </DialogBody>
                        <DialogFooter>
                            <DialogButtonFormCancel>"Cancel"</DialogButtonFormCancel>
                            <DialogButtonFormSubmit>"Save changes"</DialogButtonFormSubmit>
                        </DialogFooter>
                    </DialogForm>
                </DialogContent>
            </DialogComponent>

        </DialogProvider>
    }
}

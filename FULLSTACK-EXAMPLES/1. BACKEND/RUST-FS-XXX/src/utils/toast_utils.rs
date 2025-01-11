use leptos::*;

use crate::components::toaster_custom::{
    toast::{Toast, ToastVariant},
    toast_id::ToastId,
    types::Toasts,
};

pub fn show_toast(toast_context: Toasts, variant: ToastVariant, message: &str) {
    let toast_id = ToastId::new();
    toast_context.toast(
        view! { <Toast toast_id variant=variant title=message.to_string().into_view() /> },
        Some(toast_id),
        None,
    );
}

pub fn handle_error_toast(toast_context: Toasts) {
    show_toast(
        toast_context,
        ToastVariant::Destructive,
        "🔸 An error occurred",
    );
}

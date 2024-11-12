use leptos::*;

// use crate::components::form_examples::form_error_boundary_complex::FormErrorBoundaryComplex;
// use crate::components::form_examples::form_error_boundary_multiple::FormErrorBoundaryMultiple;
use crate::components::form_examples::form_uncontrolled::FormUncontrolled;
use crate::components::form_examples::form_validator::FormValidator;
use crate::components::form_examples::form_validator_display::FormValidatorDisplay;
use crate::components::form_examples::form_validator_display_disable::FormValidatorDisplayDisable;

#[component]
pub fn PageForms() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">

            <FormUncontrolled />

            // <FormErrorBoundaryComplex />

            // <FormErrorBoundaryMultiple />

            <FormValidator />

            <FormValidatorDisplay />

            <FormValidatorDisplayDisable />

        </div>
    }
}

use leptos::*;
use leptos_use::{use_intl_number_format, NumberStyle, UseIntlNumberFormatOptions};

use crate::registry::ui::slider::Slider;

#[component]
pub fn DemoUseIntlNumberFormat() -> impl IntoView {
    let (number, set_number) = create_signal(123456.78);

    let de_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("de-DE")
            .style(NumberStyle::Currency)
            .currency("EUR"),
    );
    let de_num = de_nf.format::<f64>(number);

    let ja_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("ja-JP")
            .style(NumberStyle::Currency)
            .currency("JPY"),
    );
    let ja_num = ja_nf.format::<f64>(number);

    let in_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("en-IN")
            .maximum_significant_digits(3),
    );
    let in_num = in_nf.format::<f64>(number);

    view! {
        <Slider
            class="max-w-[270px]"
            value=number
            on_input=set_number
            min=-1000000.0
            max=1000000.0
            step=0.01
        />
        <p>"Number: " {number}</p>
        <p>"German currency (EUR): " {de_num}</p>
        <p>"Japanese currency (JPY): " {ja_num}</p>
        <p>"Indian 3 max significant digits: " {in_num}</p>
    }
}

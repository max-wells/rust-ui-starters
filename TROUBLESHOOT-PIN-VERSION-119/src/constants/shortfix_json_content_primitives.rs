// TODO. Export this here
pub const SHORTFIX_JSON_CONTENT_TABS: &str = r#"
{
  "name": "avatar",
  "registry_dependencies": [
    "_shared",
    "primitives/avatar"
  ],
  "files": [
    {
      "name": "avatar.tsx",
      "content": "\"use client\";\n\nimport { clx } from \"@/lib/utils/clx/clx-merge\";\nimport * as AvatarPrimitive from \"@/registry/default/primitives/avatar\";\nimport { STYLES } from \"@/registry/default/ui/_shared\";\n\n//\nexport const Avatar = clx(\n  AvatarPrimitive.Root,\n  \"relative flex size-10 shrink-0 overflow-hidden rounded-full\",\n);\n\nexport const AvatarImage = clx(AvatarPrimitive.Image, STYLES.SIZE_FULL, \"aspect-square\");\n\nexport const AvatarFallback = clx(\n  AvatarPrimitive.Fallback,\n  STYLES.FLEX_CENTER_JUSTIFIED,\n  STYLES.SIZE_FULL,\n  \"rounded-full bg-muted\",\n);\n"
    }
  ],
  "type": "components:ui"
}
"#;

pub const SHORTFIX_JSON_CONTENT_BADGE: &str = r#"
{
	"name": "demo_badge",
	"files": [
		{
			"name": "demo_badge.rs",
			"content": "use leptos::*;\n\nuse crate::registry::ui::badge::Badge;\n\n#[component]\npub fn DemoBadge() -> impl IntoView {\n    view! { <Badge>\"Default\"</Badge> }\n}\n"
		}
	]
}
"#;

pub const SHORTFIX_JSON_CONTENT_BUTTON: &str = r#"
{
	"name": "demo_button",
	"files": [
		{
			"name": "demo_button.rs",
			"content": "use leptos::*;\n\nuse crate::registry::ui::button::Button;\n\n#[component]\npub fn DemoButton() -> impl IntoView {\n    view! { <Button>Button</Button> }\n}\n"
		}
	]
}
"#;

pub const SHORTFIX_JSON_CONTENT_BUTTON_REACTIVE: &str = r#"
{
	"name": "demo_button_reactive",
	"files": [
		{
			"name": "demo_button_reactive.rs",
			"content": "use leptos::*;\n\nuse crate::registry::ui::button::Button;\n\n#[component]\npub fn DemoButtonReactive() -> impl IntoView {\n    // Creates a reactive value to update the button\n    let (count, set_count) = create_signal(0);\n    let on_click = move |_| set_count.update(|count| *count += 1);\n\n    view! { <Button on:click=on_click>\"Click Me: \" {count}</Button> }\n}\n"
		}
	]
}
"#;

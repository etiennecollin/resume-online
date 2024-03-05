use leptos::*;

#[component]
pub fn Spacer(#[prop(into)] width: u8, #[prop(into)] height: u8) -> impl IntoView {
    view! { <div class=format!("mx-{} my-{}", width, height)></div> }
}

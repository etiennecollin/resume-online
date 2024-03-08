use leptos::*;

#[component]
pub fn Social(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    #[prop(into)] fa_icon: String,
) -> impl IntoView {
    view! {
        <a href=href aria-label=label>
            <i class=format!(
                "{fa_icon} text-4xl text-light-600 hover:text-accentlight-600 dark:text-dark-400 hover:dark:text-accentdark-400",
            )></i>
        </a>
    }
}

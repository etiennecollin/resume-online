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
                "{fa_icon} text-4xl text-lighttext-600 hover:text-lightaccent-600 dark:text-darktext-400 hover:dark:text-darkaccent-400",
            )></i>
        </a>
    }
}

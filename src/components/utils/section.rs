use leptos::*;

#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="bg-light-200 dark:bg-dark-800 rounded-xl py-2 p-3 lg:py-3 lg:p-5 print:break-before-all print:break-after-all print:break-inside-avoid">
            <h2 class="text-xl text-accentlight-600 dark:text-accentdark-400">{title}</h2>
            <div class="grid gap-2 pt-2">{children().nodes}</div>
        </section>
    }
}

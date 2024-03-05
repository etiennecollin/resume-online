use leptos::*;

#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="bg-slate-200 dark:bg-slate-800 rounded-xl py-2 p-3 md:py-3 md:p-5">
            <h2 class="text-xl text-slate-900 dark:text-slate-100">{title}</h2>
            <div class="grid gap-2 pt-2">{children().nodes}</div>
        </section>
    }
}

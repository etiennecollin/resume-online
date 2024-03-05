use leptos::*;

#[component]
pub fn Subsection(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="bg-slate-300 dark:bg-slate-700 rounded-xl py-2 p-3 md:py-3 md:p-5">
            <h2 class="text-xl text-slate-800 dark:text-slate-200">{title}</h2>
            <div class="grid gap-2 pt-2">{children().nodes}</div>
        </section>
    }
}

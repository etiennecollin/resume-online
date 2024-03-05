use leptos::*;

use crate::utils::empty::Empty;

#[component]
pub fn Award(
    #[prop(into)] title: String,
    #[prop(into)] year: String,
    #[prop(into)] description: String,
    #[prop(into)] location: String,
) -> impl IntoView {
    view! {
        <section class="bg-slate-300 dark:bg-slate-700 rounded-xl px-2 md:px-5">
            <div class="flex justify-between">
                <div class="flex flex-row flex-auto gap-6 h-full">
                    <p class="text-slate-600 dark:text-slate-400">{year}</p>
                    <div class="flex flex-row flex-auto gap-1 h-full">
                        <p class="text-slate-700 dark:text-slate-300">{title} ","</p>
                        <p class="text-slate-600 dark:text-slate-400">{description}</p>
                    </div>
                </div>
                <p class="text-slate-600 dark:text-slate-300">{location}</p>
            </div>
        </section>
    }
}

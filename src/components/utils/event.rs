use leptos::*;

use crate::utils::empty::Empty;

#[component]
pub fn Event(
    #[prop(into)] title: String,
    #[prop(into)] subtitle: String,
    #[prop(into)] date: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] list: Option<Vec<String>>,
) -> impl IntoView {
    let (description_signal, _) = create_signal(description);
    let (list_signal, _) = create_signal(list);

    view! {
        <section class="bg-slate-300 dark:bg-slate-700 rounded-xl py-2 px-5 grid">
            <div>
                <div class="flex justify-between">
                    <h3 class="text-xl text-slate-800 dark:text-slate-200">{title}</h3>
                    <p class="text-slate-700 dark:text-slate-300">{date}</p>
                </div>
                <h4 class="text-slate-700 dark:text-slate-300">{subtitle}</h4>
            </div>
            <div class="px-1">
                <Show when=move || { description_signal.get().is_some() } fallback=|| view! { <Empty/> }>
                    <p class="text-slate-600 dark:text-slate-400">{description_signal.get().unwrap()}</p>
                </Show>
                <Show when=move || { list_signal.get().is_some() } fallback=|| view! { <Empty/> }>
                    <ul class="list-disc list-inside">
                        {list_signal
                            .get()
                            .unwrap()
                            .into_iter()
                            .map(|item| view! { <li class="text-slate-600 dark:text-slate-400">{item}</li> })
                            .collect::<Vec<_>>()}
                    </ul>
                </Show>
            </div>

        </section>
    }
}

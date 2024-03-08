use leptos::*;

#[component]
pub fn Award(
    #[prop(into)] title: String,
    #[prop(into)] year: String,
    #[prop(into)] description: String,
    #[prop(into)] location: String,
) -> impl IntoView {
    view! {
        <section class="bg-light-300 dark:bg-dark-700 rounded-xl px-2 lg:px-5">
            <div class="flex justify-between">
                <div class="flex flex-row flex-auto gap-6 h-full">
                    <p class="text-light-600 dark:text-dark-400">{year}</p>
                    <div class="flex flex-row flex-auto gap-1 h-full">
                        <p class="text-light-700 dark:text-dark-300">{title} ","</p>
                        <p class="text-light-600 dark:text-dark-400">{description}</p>
                    </div>
                </div>
                <p class="text-light-600 dark:text-dark-300">{location}</p>
            </div>
        </section>
    }
}

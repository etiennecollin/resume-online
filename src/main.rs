use leptos::*;
use leptos_meta::*;
use online_resume::app::App;

fn main() {
    provide_meta_context();
    leptos::mount_to_body(|| {
        view! {
            <main class="mx-auto px-12 bg-stone-100 dark:bg-slate-900">
                <App/>
            </main>
        }
    });
}

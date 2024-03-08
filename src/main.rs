use leptos::*;
use leptos_meta::*;
use resume_online::app::App;

fn main() {
    provide_meta_context();
    leptos::mount_to_body(|| {
        view! {
            <main class="mx-auto px-12 bg-light-100 dark:bg-dark-900">
                <App/>
            </main>
        }
    });
}

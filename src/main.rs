use leptos::*;
use leptos_meta::*;
use online_resume::app::App;

fn main() {
    provide_meta_context();
    leptos::mount_to_body(|| {
        view! {
            <main class="h-screen w-full bg-slate-100 dark:bg-slate-900">
                <div class="mx-auto px-12 ">
                    <App/>
                </div>
            </main>
        }
    });
}

use crate::utils::social::Social;
use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <h1 class="text-5xl font-semibold text-slate-900 dark:text-slate-100">"Hi, im Etienne"</h1>
        <img class="rounded-full w-auto m-8" src="assets/images/profile.jpg" alt="Profile picture"/>
        <h2 class="pt-2 text-xl text-slate-900 dark:text-slate-100">"Student & Schulich Leader"</h2>
        <a class="text-slate-800 dark:text-slate-200" href="mailto:collin.etienne.contact@gmail.com">
            "collin.etienne.contact@gmail.com"
        </a>
        <p class="pt-4 grow text-slate-800 dark:text-slate-200">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec placerat justo neque, ut accumsan mi tristique in."
        </p>
        <div class="grid gap-5 grid-cols-3 mt-8">
            <Social href="https://github.com/etiennecollin" label="Checkout my GitHub" fa_icon="fa-brands fa-github"/>
            <Social
                href="https://www.linkedin.com/in/etiennecollin"
                label="Checkout my LinkedIn"
                fa_icon="fa-brands fa-linkedin"
            />
            <Social href="https://etiennecollin.com" label="Checkout my website" fa_icon="fa-solid fa-globe"/>
        </div>
    }
}

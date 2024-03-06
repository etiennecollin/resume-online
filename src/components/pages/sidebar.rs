use crate::utils::social::Social;
use leptos::*;
use web_sys::HtmlElement;

#[component]
pub fn Sidebar() -> impl IntoView {
    let title = "<h1 class=\"text-5xl font-semibold text-slate-900 dark:text-slate-100\">Hi, im Etienne</h1>";
    let subtitle = "<h2 class=\"pt-2 text-xl text-slate-900 dark:text-slate-100\">Student & Schulich Leader</h2>";
    let contact = "<a class=\"text-slate-800 dark:text-slate-200\" href=\"mailto:collin.etienne.contact@gmail.com\">collin.etienne.contact@gmail.com</a>";
    let description = "<p class=\"pt-2 grow text-slate-800 dark:text-slate-200\">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec placerat justo neque, ut accumsan mi tristique in.</p>";
    let image = "<img class=\"rounded-full w-1/3 lg:w-auto mt-8 mb-2 lg:m-8 mx-auto\" src=\"assets/images/profile.jpg\" alt=\"Profile picture\"/>";

    let small = format!(
        "<div>{}{}{}{}</div>{}",
        title, subtitle, contact, description, image
    );
    let large = format!("{}{}{}{}{}", title, image, subtitle, contact, description);

    view! {
        <div class="lg:my-auto flex flex-col items-start grow">
            <div class="flex flex-row justify-between items-center lg:hidden" inner_html=small></div>
            <div class="hidden justify-between items-center lg:block" inner_html=large></div>
            <div class="flex flex-row gap-5 lg:mt-8">
                <Social
                    href="https://github.com/etiennecollin"
                    label="Checkout my GitHub"
                    fa_icon="fa-brands fa-github"
                />
                <Social
                    href="https://www.linkedin.com/in/etiennecollin"
                    label="Checkout my LinkedIn"
                    fa_icon="fa-brands fa-linkedin"
                />
                <Social href="https://etiennecollin.com" label="Checkout my website" fa_icon="fa-solid fa-globe"/>
            </div>
        </div>
    }
}

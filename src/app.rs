use leptos::*;
use leptos_meta::*;

use crate::pages::{
    about::About, awards::Awards, education::Education, experience::Experience,
    extracurricular::Extracurricular, sidebar::Sidebar,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <div class="grid gap-5 grid-cols-1 lg:grid-cols-5 h-screen">
            <Sidebar/>
            <div class="flex flex-col flex-auto mx-auto lg:col-span-4 lg:w-11/12">
                <div class="flex flex-col flex-auto gap-3 pb-4 lg:h-0 lg:py-4 lg:overflow-y-scroll lg:no-scrollbar">
                    <About/>
                    <Education/>
                    <Experience/>
                    <Extracurricular/>
                    <Awards/>
                </div>
            </div>
        </div>
    }
}

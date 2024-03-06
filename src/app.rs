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
            <div class="lg:col-span-4 flex flex-col flex-auto lg:w-11/12 mx-auto">
                <div class="flex flex-col flex-auto gap-3 lg:overflow-y-scroll lg:h-0 lg:no-scrollbar pb-4 lg:py-4">
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

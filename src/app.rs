use leptos::*;
use leptos_meta::*;

use crate::{
    pages::{
        about::About, awards::Awards, education::Education, experience::Experience,
        extracurricular::Extracurricular, sidebar::Sidebar,
    },
    utils::spacer::Spacer,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <div class="grid gap-5 grid-cols-1 md:grid-cols-5 h-screen">
            <div class="mt-16 md:my-16 md:col-span-1 flex flex-col items-start grow">
                <Sidebar/>
            </div>
            <div class="md:col-span-4 flex flex-col flex-auto md:w-11/12 mx-auto">
                <div class="flex flex-col flex-auto gap-3 md:overflow-y-scroll md:h-0 md:no-scrollbar">
                    <Spacer width=0 height=16/>
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

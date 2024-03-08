use crate::utils::{section::Section, subsection::Subsection};
use leptos::*;

#[component]
pub fn Awards() -> impl IntoView {
    view! {
        <Section title="Honors & Awards">
            <Subsection>
                <h2 class="text-xl text-lighttext-800 dark:text-darktext-200">"National Awards"</h2>
                <Award
                    title="Schulich Leader Scholarship"
                    year="2022"
                    description="One of the fifty 80,000$ scholarships awarded in science throughout Canada".to_owned()
                    location="Canada"
                />
            </Subsection>
            <Subsection>
                <h2 class="text-xl text-lighttext-800 dark:text-darktext-200">"Other Awards"</h2>
                <Award
                    title="American Express Excellence Scholarship"
                    year="2024"
                    description="$500 scholarship".to_owned()
                    location="Montréal, Canada"
                />
                <Award
                    title="Lambda Research Scholarship"
                    year="2023"
                    description="$8,000 research scholarship at Ciela/Mila".to_owned()
                    location="Montréal, Canada"
                />
                <Award
                    title="Bourses d’Excellence des Diplômés et Professeurs du DIRO"
                    year="2023"
                    description="$2,500 scholarship".to_owned()
                    location="Montréal, Canada"
                />
                <Award
                    title="Admission Scholarship"
                    year="2023"
                    description="$5,000 scholarship".to_owned()
                    location="Montréal, Canada"
                />
                <Award
                    title="Beneva Resilience Award"
                    year="2022"
                    description="$2,500 scholarship".to_owned()
                    location="Québec, Canada"
                />
                <Award
                    title="Cultural Award"
                    year="2022"
                    description="$700 award".to_owned()
                    location="Québec, Canada"
                />
                <Award
                    title="Forces AVENIR Finalist"
                    year="2022"
                    description="$700 scholarship".to_owned()
                    location="Québec, Canada"
                />
            </Subsection>
        </Section>
    }
}

#[component]
pub fn Award(
    #[prop(into)] title: String,
    #[prop(into)] year: String,
    #[prop(into)] description: String,
    #[prop(into)] location: String,
) -> impl IntoView {
    view! {
        <section class="flex justify-between px-2 lg:px-5">
            <div class="flex flex-row flex-auto gap-6 h-full">
                <p class="text-lighttext-600 dark:text-darktext-400">{year}</p>
                <div class="flex flex-row flex-auto gap-1 h-full">
                    <p class="text-lighttext-700 dark:text-darktext-300">{title} ","</p>
                    <p class="text-lighttext-600 dark:text-darktext-400">{description}</p>
                </div>
            </div>
            <p class="text-lighttext-700 dark:text-darktext-300">{location}</p>
        </section>
    }
}

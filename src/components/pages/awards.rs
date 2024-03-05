use crate::utils::{award::Award, section::Section, subsection::Subsection};
use leptos::*;

#[component]
pub fn Awards() -> impl IntoView {
    view! {
        <Section title="Honors & Awards">
            <Subsection title="National Awards">
                <Award
                    title="Schulich Leader Scholarship"
                    year="2022"
                    description="One of the fifty 80,000$ scholarships awarded in science throughout Canada".to_owned()
                    location="Canada"
                />
            </Subsection>
            <Subsection title="Other Awards">
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

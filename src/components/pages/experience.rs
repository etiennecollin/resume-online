use crate::utils::{event::Event, section::Section};
use leptos::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <Section title="Experiences">
            <Event
                title="Teacher Assistant"
                subtitle="Université de Montréal"
                date="Sep 2023 - Jan 2024"
                description="Teacher assistant for Professor Marc Feeley in the 320+ students Introduction to Programming class (IFT1015) at Université de Montréal. Taught programming fundamentals and good practices using Python as a tool."
                    .to_owned()
                list=vec![
                    "Answered questions through forums and emails".to_owned(),
                    "Graded homework, quizzes and projects, and gave feedback to students".to_owned(),
                    "Presented examples and exercises complementary to the professor’s teachings".to_owned(),
                    "Wrote scripts to facilitate the grading process".to_owned(),
                ]
            />

        </Section>
    }
}

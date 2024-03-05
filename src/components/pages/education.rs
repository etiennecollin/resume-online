use leptos::*;

use crate::utils::{event::Event, section::Section};

#[component]
pub fn Education() -> impl IntoView {
    view! {
        <Section title="Education">
            <Event
                title="Université de Montréal"
                subtitle="B.Sc., Computer Science"
                date="May 2022 - Aug 2025"
                list=vec!["Current cumulative GPA: 4.211/4.3".to_owned()]
            />

            <Event
                title="CEGEP Champlain St. Lawrence"
                subtitle="DEC, Pure and Applied Science Program"
                date="Sept 2020 - Jun 2022"
                list=vec![
                    "Grade: 91.577% | R-Score: 36.885".to_owned(),
                    "Graduated with mentions in Mathematics (Science Program), Humanities and Philosophy, English and Physical Education."
                        .to_owned(),
                    "3x on the Dean’s Honour List (semester with 90%+ general average) | 1x on the Honour List (semester with 85%+ general average)"
                        .to_owned(),
                ]
            />

        </Section>
    }
}

use crate::utils::{event::Event, section::Section};
use leptos::*;

#[component]
pub fn Extracurricular() -> impl IntoView {
    view! {
        <Section title="Extracurricular Activities">
            <Event
                title="Participant"
                subtitle="AI Helps Ukraine"
                date="Aug 2023"
                list=vec![
                    "Selected as one of 15 participants worldwide to Astromatic, a week‑long program about AI, ML, and astrophysics. The program consisted of lectures on these topics given by experts, followed by a hackathon and a competition."
                        .to_owned(),
                    "As part of a team of four, won the Jury prize for implementing a way to denoise, deblur and super‑res galaxy images up to the quality of the HST (Hubble Space Telescope) at the end of a 48h hackathon using custom bayesian score‑based diffusion models."
                        .to_owned(),
                ]
            />

        </Section>
    }
}

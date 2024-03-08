use crate::utils::section::Section;
use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Section title="About Him">
            <p class="text-light-700 dark:text-dark-300">
                "Meet Etienne, an enthusiastic Computer Science student characterized by an insatiable curiosity about the world. Etienne is excited to explore the boundless potential of AI and is currently on a journey towards obtaining a Masters degree, and possibly a PhD, in this field. The elegant interplay of mathematics and statistics that underpin AI fascinate Etienne, propelling his desire to unravel its inner workings, contribute to it meaningfully, and leverage it for positive change."
            </p>
        </Section>
    }
}

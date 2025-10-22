use super::Card;
use leptos::prelude::*;

#[component]
pub fn SimpleCardMetrics(
    title: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="mb-8">
            <Card
                padded=true
                header=view!{
                    <h3 class="text-lg font-medium">{title}</h3>
                    <p class="mt-1 text-neutral-400">{description}</p>
                }.into_any()
            >

            <div class="mt-2 text-3xl font-semibold text-[#FFFBEB] text-right">
               { children() }
            </div>

            </Card>
        </section>
    }
}

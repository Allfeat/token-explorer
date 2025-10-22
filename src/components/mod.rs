use leptos::prelude::*;

pub mod fetchable_balance;
pub mod footer;
pub mod header;
pub mod metrics_cards;
pub mod simple_card_metrics;
pub mod toast;

#[component]
pub fn Card(
    #[prop(optional)] header: Option<AnyView>,
    #[prop(optional)] footer: Option<AnyView>,
    #[prop(optional)] padded: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let padding = if padded { "p-6 sm:p-8" } else { "" };

    view! {
    <section class=format!(
    "relative overflow-hidden rounded-3xl border border-white/10 bg-[#0f1110] shadow-[0_12px_40px_-16px_rgba(0,0,0,0.6)] {}",
    class
    )>
    <div aria-hidden="true" class="pointer-events-none absolute -left-16 -top-16 h-44 w-44 rounded-full bg-[radial-gradient(closest-side,rgba(244,63,94,0.18),rgba(244,63,94,0)_70%)] blur-2xl"></div>
    <div aria-hidden="true" class="pointer-events-none absolute -right-20 -bottom-20 h-52 w-52 rounded-full bg-[radial-gradient(closest-side,rgba(16,185,129,0.18),rgba(16,185,129,0)_70%)] blur-2xl"></div>


    { header.map(|h| view! {
    <div class=format!("{} border-b border-white/10", padding)>{h}</div>
    }) }


    <div class=padding>
    { children() }
    </div>


    { footer.map(|f| view! {
    <div class=format!("{} border-t border-white/10", padding)>{f}</div>
    }) }
    </section>
    }
}

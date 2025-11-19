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
    #[prop(default = true)] padded: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let padding_class = if padded { "p-6" } else { "" };

    view! {
        <div class=format!(
            "relative group isolate flex flex-col overflow-hidden rounded-2xl border border-white/5 bg-[#0F0F0F] shadow-lg transition-all hover:border-white/10 hover:shadow-xl {}",
            class
        )>
            // --- 1. Background Effects ---

            <div class="absolute inset-0 -z-10 bg-gradient-to-br from-emerald-500/[0.03] to-transparent opacity-0 transition-opacity duration-500 group-hover:opacity-100"></div>

            <div class="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-white/10 to-transparent"></div>


            // --- 2. Content Structure ---

            { header.map(|h| view! {
                <div class=format!("{} border-b border-white/5 bg-white/[0.01]", padding_class)>
                    {h}
                </div>
            }) }

            <div class=format!("flex-1 relative {}", padding_class)>
                { children() }
            </div>

            { footer.map(|f| view! {
                <div class=format!("{} border-t border-white/5 bg-white/[0.01]", padding_class)>
                    {f}
                </div>
            }) }
        </div>
    }
}

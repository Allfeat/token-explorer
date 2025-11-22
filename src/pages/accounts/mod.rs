use crate::{
    components::toast::use_toast,
    pages::accounts::{allocations::AccountAllocations, balances::AccountBalances},
    utils::ss58_identicon_svg,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod allocations;
mod balances;

#[component]
pub fn Account() -> impl IntoView {
    let toast = use_toast();
    let params = use_params_map();

    let address = Memo::new(move |_| params.read().get("id").unwrap_or_default());

    let copy_to_clipboard = move |_| {
        let addr = address.get();
        let _ = window().navigator().clipboard().write_text(&addr);

        (toast.add_toast)("Address copied to clipboard".to_string());
    };

    view! {
        <div class="flex flex-col gap-8 sm:gap-12 fade-in">

            // --- HEADER IDENTITY ---
            <section class="relative pt-2 sm:pt-6">
                <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 sm:gap-6">

                    <div class="relative group shrink-0">
                        <div class="absolute -inset-0.5 bg-gradient-to-br from-emerald-500 to-cyan-500 rounded-full opacity-30 blur group-hover:opacity-60 transition duration-500"></div>
                        <div class="relative h-12 w-12 sm:h-16 sm:w-16 rounded-full bg-[#050505] ring-2 ring-white/10 flex items-center justify-center overflow-hidden">
                             <div inner_html=move || ss58_identicon_svg(&address.get(), 64) />
                        </div>
                    </div>

                    <div class="flex flex-col gap-1 min-w-0 w-full">
                        <h1 class="text-[10px] sm:text-xs font-bold uppercase tracking-widest text-emerald-500 mb-0.5 sm:mb-1">
                            "Account Details"
                        </h1>

                        <button
                            class="group flex items-center gap-2 sm:gap-3 text-left transition-all active:scale-[0.98] w-full"
                            on:click=copy_to_clipboard
                            title="Click to copy"
                        >
                            <span class="text-lg sm:text-3xl font-mono font-bold text-white break-all hover:text-emerald-50 transition-colors leading-tight">
                                { move || address.get() }
                            </span>

                            <div class="flex h-7 w-7 sm:h-8 sm:w-8 items-center justify-center rounded-full bg-white/5 text-neutral-400 opacity-100 sm:opacity-0 sm:group-hover:opacity-100 transition-opacity shrink-0">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                            </div>
                        </button>
                    </div>
                </div>
            </section>

            <AccountBalances id=address.get() />
            <AccountAllocations id=address.get() />
        </div>
    }
}

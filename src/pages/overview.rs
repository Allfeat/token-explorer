use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use web_sys::SubmitEvent;

use crate::components::metrics_cards::{CirculatingSupply, TotalIssuance, TreasuryBalance};

#[component]
pub fn Overview() -> impl IntoView {
    let navigate = use_navigate();
    let address = RwSignal::new(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let addr = address.get().trim().to_string();
        if !addr.is_empty() {
            navigate(&format!("/accounts/{}", addr), Default::default());
        }
    };

    view! {
        <div class="flex flex-col gap-8 sm:gap-16">

            // --- HERO SECTION ---
            <section class="relative flex flex-col items-center text-center pt-2 sm:pt-8">

                <h1 class="text-4xl sm:text-6xl font-extrabold tracking-tight">
                    <span class="bg-gradient-to-b from-white to-white/60 bg-clip-text text-transparent">
                        "Allfeat Economy"
                    </span>
                    <br />
                    <span class="text-3xl sm:text-5xl text-neutral-500 font-bold mt-2 block">
                        "Explorer"
                    </span>
                </h1>

                <p class="mt-6 text-base sm:text-lg text-neutral-400 max-w-xl leading-relaxed px-4">
                    "Track the heartbeat of the network. Analyze token movements, allocations, and supply metrics in real-time."
                </p>

                // --- SEARCH MODULE ---
                <div class="w-full max-w-2xl mt-8 sm:mt-10 relative group px-2 sm:px-0">
                    <div class="absolute -inset-1 bg-gradient-to-r from-emerald-500/20 to-cyan-500/20 rounded-[2rem] blur opacity-20 group-hover:opacity-40 transition duration-500"></div>

                    <form on:submit=on_submit class="relative">
                        <div class="relative flex items-center overflow-hidden rounded-[2rem] border border-white/10 bg-[#0F0F0F] shadow-2xl transition-all focus-within:border-emerald-500/50 focus-within:ring-1 focus-within:ring-emerald-500/20">

                            <div class="pl-4 sm:pl-5 text-neutral-500">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="11" cy="11" r="8"></circle>
                                    <path d="m21 21-4.3-4.3"></path>
                                </svg>
                            </div>

                            <input
                                type="text"
                                autocomplete="off"
                                spellcheck="false"
                                placeholder="Search by account address (qGx...)"
                                class="h-12 sm:h-14 w-full bg-transparent px-3 sm:px-4 text-base text-white placeholder:text-neutral-600 focus:outline-none"
                                prop:value=address
                                on:input=move |ev| address.set(event_target_value(&ev))
                            />

                            <div class="pr-1.5 sm:pr-2">
                                <button
                                    type="submit"
                                    class="flex h-9 w-9 sm:h-10 sm:w-10 items-center justify-center rounded-full bg-white/5 text-neutral-400 transition-colors hover:bg-emerald-500 hover:text-white active:scale-95"
                                    aria-label="Search"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </form>

                    <div class="mt-3 flex items-center justify-center gap-2 text-xs text-neutral-600">
                        <span class="rounded border border-white/5 bg-white/5 px-1.5 py-0.5 font-mono text-[10px] hidden sm:inline">"ENTER"</span>
                        <span class="hidden sm:inline">"to search"</span>

                        <span class="sm:hidden">"Tap go to search"</span>
                    </div>
                </div>
            </section>

            // --- METRICS GRID ---
            <section>
                <div class="flex items-center gap-2 mb-4 sm:mb-6 px-1">
                    <div class="h-1 w-1 rounded-full bg-emerald-500"></div>
                    <h2 class="text-sm font-mono uppercase tracking-wider text-neutral-400">
                        "Network Overview"
                    </h2>
                </div>

                <div class="grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
                    <TotalIssuance />
                    <CirculatingSupply />
                    <TreasuryBalance />
                </div>
            </section>
        </div>
    }
}

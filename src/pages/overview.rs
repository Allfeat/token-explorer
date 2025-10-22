use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::metrics_cards::{CirculatingSupply, TotalIssuance, TreasuryBalance};

#[component]
pub fn Overview() -> impl IntoView {
    let navigate = use_navigate();
    let address = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let addr = address.get().trim().to_string();
        if !addr.is_empty() {
            navigate(&format!("/accounts/{}", addr), Default::default());
        }
    };

    view! {
        <section class="relative overflow-hidden p-6 sm:pt-10 pb-8 sm:pb-12 mb-10 sm:mb-14">
            <div aria-hidden="true" class="pointer-events-none absolute inset-0 -z-10">
                <div class="absolute left-1/2 top-[-30%] h-[640px] w-[640px] -translate-x-1/2
                            bg-[radial-gradient(closest-side,rgba(16,185,129,0.14),rgba(16,185,129,0)_70%)]
                            blur-3xl [mask-image:radial-gradient(closest-side,black,transparent_70%)]"></div>
                <div class="absolute right-[-20%] bottom-[-25%] h-[520px] w-[520px]
                            bg-[radial-gradient(closest-side,rgba(124,58,237,0.10),transparent_70%)]
                            blur-3xl [mask-image:radial-gradient(closest-side,black,transparent_70%)]"></div>
            </div>

            <div class="relative flex flex-col gap-5">
                <h1 class="text-4xl sm:text-5xl font-extrabold leading-tight text-neutral-50">
                    "Allfeat Economy Explorer"
                </h1>

                <p class="text-base sm:text-lg text-neutral-400 max-w-2xl">
                    "View key metrics and track the network economy in real time."
                </p>

                <form class="mt-1 max-w-2xl" on:submit=on_submit>
                    <div
                        class="relative rounded-[1.75rem] border border-white/12 bg-white/[0.045]
                               shadow-[inset_0_1px_0_rgba(255,255,255,0.04)]
                               px-4 py-2 sm:px-5 sm:py-3 backdrop-blur
                               focus-within:ring-2 focus-within:ring-emerald-400/25"
                    >
                        <input
                            type="text"
                            inputmode="text"
                            autocomplete="off"
                            autocapitalize="off"
                            spellcheck="false"
                            placeholder="Search for an account details…"
                            class="w-full bg-transparent text-neutral-100 placeholder:text-neutral-500
                                   outline-none text-sm sm:text-base pr-14 sm:pr-16 pl-2 sm:pl-3"
                            prop:value=address
                            on:input=move |ev| address.set(event_target_value(&ev))
                        />

                        <button
                            type="submit"
                            class="absolute right-1.5 top-1/2 -translate-y-1/2
                                   h-9 w-9 sm:h-10 sm:w-10 grid place-items-center rounded-full
                                   border border-white/18 bg-white/[0.08]
                                   hover:bg-white/[0.12] active:scale-95 transition
                                   text-neutral-100"
                            aria-label="Search"
                            title="Search"
                        >
                            <svg viewBox="0 0 24 24" class="h-4.5 w-4.5" fill="none" stroke="currentColor">
                                <path d="M5 12h12M13 6l6 6-6 6" stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5"/>
                            </svg>
                        </button>
                    </div>
                    <p class="m-2 text-xs text-neutral-500">
                        "Enter a valid Allfeat address account, then hit Enter."
                    </p>
                </form>
            </div>
        </section>

        <section class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            <TotalIssuance />
            <CirculatingSupply />
            <TreasuryBalance />
        </section>
    }
}

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{
    components::toast::use_toast,
    pages::accounts::{allocations::AccountAllocations, balances::AccountBalances},
    utils::ss58_identicon_svg,
};

mod allocations;
mod balances;

#[component]
pub fn Account() -> impl IntoView {
    let toast = use_toast();

    let params = use_params_map();
    let address = move || params.read().get("id").unwrap_or_default();

    view! {
        <section class="relative mb-8 sm:mb-10">
            <div class="flex items-center justify-between gap-4">
                <div class="flex items-center gap-4">
                    <div
                        class="h-12 w-12 rounded-full overflow-hidden ring-1 ring-white/10 bg-black/40 cursor-pointer select-none"
                        title="Click to copy address"
                        role="button"
                        on:click= move |_| {
                            let _ = window().navigator().clipboard().write_text(&address());
                            (toast.add_toast)("Copied to clipboard".to_string())
                        }
                        tabindex="0"
                    >
                        <div inner_html=move || ss58_identicon_svg(&address(), 48) />
                    </div>

                    <div>
                        <h1 class="text-xl sm:text-2xl font-semibold text-neutral-50">"Account details"</h1>
                        <p class="text-neutral-400 text-sm break-all">{ address() }</p>
                    </div>
                </div>
            </div>
        </section>

        <AccountBalances id=address() />
        <AccountAllocations id=address() />
    }
}

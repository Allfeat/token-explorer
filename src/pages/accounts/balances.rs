use crate::{
    components::{Card, fetchable_balance::FetchableData},
    get_balance_of,
    utils::format_balance,
};
use leptos::prelude::*;

#[component]
pub fn AccountBalances(id: String) -> impl IntoView {
    let balance = OnceResource::new(get_balance_of(id));

    view! {
        <section>
             <div class="flex items-center gap-2 mb-4 sm:mb-6">
                <div class="h-1 w-1 rounded-full bg-emerald-500"></div>
                <h2 class="text-sm font-mono uppercase tracking-wider text-neutral-400">
                    "Balances"
                </h2>
            </div>

            <div class="grid gap-4 sm:gap-6 lg:grid-cols-3">

                // --- 1. TRANSFERABLE ---
                <Card class="lg:col-span-2 relative overflow-hidden border-emerald-500/20 min-h-[160px]">
                    <div class="absolute top-0 right-0 -mt-4 -mr-4 h-32 w-32 rounded-full bg-emerald-500/10 blur-3xl"></div>

                    <div class="flex flex-col h-full justify-between gap-4 sm:gap-6">
                        <div class="flex items-center justify-between">
                            <span class="text-xs sm:text-sm font-medium text-neutral-400 uppercase tracking-wider">"Transferable"</span>
                            <span class="flex h-2 w-2 rounded-full bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.6)]"></span>
                        </div>

                        <FetchableData data=balance render=move |value| {
                            view! {
                                <div>
                                    <div class="text-3xl sm:text-5xl font-mono font-bold text-white tracking-tight break-all">
                                        { format_balance(value.free, true) }
                                    </div>
                                    <p class="mt-2 text-xs sm:text-sm text-neutral-500">
                                        "Available for transfers and transaction fees."
                                    </p>
                                </div>
                            }.into_any()
                        } />
                    </div>
                </Card>

                // --- 2. LOCKED & FROZEN ---
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-1 gap-4 sm:gap-6">
                    // Locked
                    <Card class="">
                        <div class="flex items-center justify-between mb-2 sm:mb-4">
                            <span class="text-[10px] sm:text-xs font-medium text-neutral-500 uppercase tracking-wider">"Locked"</span>
                            <svg class="text-rose-500/50 w-4 h-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
                        </div>
                         <FetchableData data=balance render=move |value| {
                            view! {
                                <div class="text-xl sm:text-2xl font-mono font-bold text-white truncate" title=format_balance(value.reserved, true)>
                                    { format_balance(value.reserved, true) }
                                </div>
                            }.into_any()
                        } />
                    </Card>

                    // Frozen
                    <Card class="">
                        <div class="flex items-center justify-between mb-2 sm:mb-4">
                            <span class="text-[10px] sm:text-xs font-medium text-neutral-500 uppercase tracking-wider">"Frozen"</span>
                            <svg class="text-amber-500/50 w-4 h-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="2" x2="12" y2="22"></line><line x1="12" y1="22" x2="20" y2="16"></line><line x1="12" y1="22" x2="4" y2="16"></line><line x1="12" y1="2" x2="20" y2="8"></line><line x1="12" y1="2" x2="4" y2="8"></line><line x1="20" y1="16" x2="20" y2="8"></line><line x1="4" y1="16" x2="4" y2="8"></line></svg>
                        </div>
                         <FetchableData data=balance render=move |value| {
                            view! {
                                <div class="text-xl sm:text-2xl font-mono font-bold text-white truncate" title=format_balance(value.frozen, true)>
                                    { format_balance(value.frozen, true) }
                                </div>
                            }.into_any()
                        } />
                    </Card>
                </div>
            </div>
        </section>
    }
}

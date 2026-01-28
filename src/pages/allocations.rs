use crate::{
    EnvelopeAllocation,
    components::{Card, fetchable_balance::FetchableData, toast::use_toast},
    get_allocations,
    utils::{blocks_to_human_duration, blocks_to_str, display_address, format_balance, ss58_identicon_svg},
};
use leptos::prelude::*;

#[component]
pub fn Allocations() -> impl IntoView {
    let allocations = OnceResource::new(get_allocations());

    view! {
        <div class="flex flex-col gap-6 sm:gap-12">

            // --- HEADER ---
            <header class="flex flex-col gap-3 sm:gap-4 pt-2 sm:pt-8">
                <h1 class="text-3xl sm:text-4xl font-extrabold tracking-tight">
                    <span class="bg-gradient-to-b from-white to-white/60 bg-clip-text text-transparent">
                        "Token Sources"
                    </span>
                </h1>
                <p class="text-neutral-400 max-w-2xl leading-relaxed text-sm sm:text-base">
                    "Detailed breakdown of network emission sources. Track vesting schedules, cliffs, and real-time distribution progress for each envelope."
                </p>
            </header>

            // --- GRID ---
            <FetchableData
                data=allocations
                render={move |items: Vec<EnvelopeAllocation>| {
                    view! {
                        <div class="grid gap-4 sm:gap-6 md:grid-cols-2 xl:grid-cols-3">
                            <For
                                each=move || items.clone()
                                key=|env| env.id.clone()
                                children=move |env| {
                                    view! { <AllocationCard env /> }
                                }
                            />
                        </div>
                    }.into_any()
                }}
            />
        </div>
    }
}

#[component]
pub fn AllocationCard(env: EnvelopeAllocation) -> impl IntoView {
    let toast = use_toast();

    let upfront_amount = env.total_cap.saturating_mul(env.upfront_rate as u128) / 100u128;
    let total = env.total_cap;
    let distributed = env.distributed.min(total);
    let remaining = total.saturating_sub(distributed);

    let distributed_pct = if total == 0 {
        0.0
    } else {
        (distributed as f64 / total as f64) * 100.0
    }
    .clamp(0.0, 100.0);

    let progress_style = format!("width: {:.2}%;", distributed_pct);

    let has_unique = env.unique_beneficiary.is_some();

    let copy_to_clipboard = move |address: &str| {
        let _ = window().navigator().clipboard().write_text(address);
        (toast.add_toast)("Address copied to clipboard".to_string());
    };

    view! {
        <Card class="h-full flex flex-col">
            <div class="flex flex-col h-full gap-5 sm:gap-6">

                // --- 1. TITLE & HEADER ---
                <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0">
                        <div class="flex items-center gap-2 mb-1">
                            <span class="text-[10px] font-bold uppercase tracking-wider text-emerald-500">
                                "Envelope"
                            </span>
                        </div>
                        <h3 class="text-base sm:text-lg font-semibold text-white leading-tight truncate pr-2">
                            { env.name.clone() }
                        </h3>
                    </div>
                    { (env.upfront_rate > 0).then(|| view! {
                        <div class="shrink-0 rounded-md bg-white/5 px-2 py-1 text-[10px] sm:text-xs font-medium text-neutral-300 border border-white/5" title="Upfront Release">
                            "Upfront " {env.upfront_rate} "%"
                        </div>
                    })}
                </div>

                // --- 2. PRIMARY KPI (Total Cap) ---
                <div>
                    <div class="text-[10px] sm:text-xs text-neutral-500 uppercase tracking-wider mb-0.5 sm:mb-1">"Total Allocation"</div>
                    <div class="text-2xl sm:text-3xl font-mono font-bold text-white tracking-tight" title=move || format_balance(env.total_cap, true)>
                        { format_balance(env.total_cap, true) }
                    </div>
                     <div class="text-xs text-neutral-500 mt-1">
                        "Upfront: " <span class="text-neutral-300">{format_balance(upfront_amount, true)}</span>
                    </div>
                </div>

                // --- 3. VESTING TIMELINE (Grid) ---
                <div class="grid grid-cols-2 gap-3 sm:gap-4 py-4 border-y border-dashed border-white/10">
                    // Cliff
                    <div>
                        <div class="text-[10px] uppercase text-neutral-500 mb-0.5">"Cliff Period"</div>
                        <div class="text-xs sm:text-sm font-medium text-neutral-200 truncate">
                            { blocks_to_human_duration(env.cliff) }
                        </div>
                        <div class="text-[10px] text-neutral-600 font-mono mt-0.5">
                            { blocks_to_str(env.cliff) } " blocks"
                        </div>
                    </div>
                    // Vesting
                    <div>
                        <div class="text-[10px] uppercase text-neutral-500 mb-0.5">"Vesting Duration"</div>
                        <div class="text-xs sm:text-sm font-medium text-neutral-200 truncate">
                            { blocks_to_human_duration(env.vesting_duration) }
                        </div>
                        <div class="text-[10px] text-neutral-600 font-mono mt-0.5">
                            { blocks_to_str(env.vesting_duration) } " blocks"
                        </div>
                    </div>
                </div>

                // --- 4. DISTRIBUTION PROGRESS ---
                { (!has_unique).then(move || view! {
                    <div class="space-y-2 sm:space-y-3 mt-auto">
                        <div class="flex justify-between text-xs">
                            <span class="text-neutral-400">"Distributed"</span>
                            <span class="text-white font-mono">{format!("{:.1}%", distributed_pct)}</span>
                        </div>

                        <div class="h-1.5 w-full rounded-full bg-white/5 overflow-hidden">
                            <div class="h-full bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.4)]" style=progress_style></div>
                        </div>

                        <div class="flex justify-between items-baseline text-xs">
                            <span class="text-neutral-500">"Remaining"</span>
                            <span class="text-neutral-300 font-mono" title=move || format_balance(remaining, true)>
                                { format_balance(remaining, true) }
                            </span>
                        </div>
                    </div>
                })}

                // --- 5. UNIQUE BENEFICIARY (Alternative Footer) ---
                { move || env.unique_beneficiary.as_ref().map(|addr| {
                    let full_address: String = addr.clone();
                    let display_name = display_address(&full_address);
                    let copy_closure = copy_to_clipboard.clone();
                    view! {
                     <div class="mt-auto rounded-xl bg-white/[0.03] border border-white/5 p-3">
                        <div class="flex items-center gap-3 overflow-hidden">
                            <div class="h-8 w-8 rounded-full bg-black ring-1 ring-white/10 shrink-0 flex items-center justify-center">
                                // Icon
                                <div inner_html=ss58_identicon_svg(&full_address, 24) class="opacity-80" />
                            </div>

                            <div class="flex-1 min-w-0">
                                <div class="text-[10px] uppercase text-neutral-500 mb-0.5">"Allocated to"</div>
                                <div class="flex items-center gap-2">
                                    <span class="text-xs text-neutral-300 font-mono truncate block" title=full_address.clone()>
                                        {display_name}
                                    </span>
                                    // Copy Button (Minimalist)
                                    <button
                                        class="text-neutral-500 hover:text-white transition-colors"
                                        title="Copy Address"
                                        on:click=move |_| copy_closure(&full_address)
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }})}
            </div>
        </Card>
    }
}

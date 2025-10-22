use leptos::prelude::*;

use crate::{
    components::{fetchable_balance::FetchableData, Card},
    get_allocations,
    utils::{blocks_to_human_duration, blocks_to_str, format_balance, ss58_identicon_svg},
    EnvelopeAllocation,
};

#[component]
pub fn Allocations() -> impl IntoView {
    let allocations = OnceResource::new(get_allocations());

    view! {
        <section class="relative overflow-hidden pt-4 pb-6 sm:pt-8 sm:pb-8 mb-8 sm:mb-10">
            <div aria-hidden="true" class="pointer-events-none absolute inset-0">
                <div class="absolute left-1/2 top-[-30%] h-[480px] w-[480px] -translate-x-1/2
                            bg-[radial-gradient(closest-side,rgba(16,185,129,0.14),transparent_70%)]
                            blur-3xl [mask-image:radial-gradient(closest-side,black,transparent_70%)]"></div>
            </div>

            <div class="relative flex flex-col gap-3">
                <h1 class="text-3xl sm:text-4xl font-semibold tracking-tight text-neutral-50">
                    "Token Allocations"
                </h1>
                <p class="text-neutral-400 max-w-prose">
                    "Explore all emission sources (envelopes): caps, upfront, cliff & linear vesting details."
                </p>
            </div>
        </section>

        <FetchableData
            data=allocations
            render={move |items: Vec<EnvelopeAllocation>| {
                view! {
                    <div class="grid gap-6 sm:grid-cols-2 xl:grid-cols-3">
                        <For
                            each=move || items.clone()
                            key=|env| env.id.clone()
                            children=move |env| {
                                // composant item
                                view! {
                                    <AllocationCard env />
                                }
                            }
                        />
                    </div>
                }.into_any()
            }}
        />
    }
}

#[component]
pub fn AllocationCard(env: EnvelopeAllocation) -> impl IntoView {
    let upfront_pct_str = format!("{}%", env.upfront_rate);
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
    let remaining_pct = (100.0 - distributed_pct).clamp(0.0, 100.0);
    let remaining_width = format!("width: {:.2}%;", remaining_pct);

    let has_unique = env.unique_beneficiary.is_some();

    // HEADER
    let header = view! {
        <div class="flex items-center justify-between min-w-0">
            <div class="flex items-center gap-3 min-w-0">
                <span class="text-sm font-medium text-neutral-200 truncate max-w-[60%]">{ env.name.clone() }</span>
                <span class="text-[10px] px-2 py-0.5 rounded-full border border-white/10 bg-white/5 text-neutral-400 shrink-0">
                    "Envelope"
                </span>
            </div>
            <div class="text-xs text-neutral-400 min-w-0 text-right">
                <span class="shrink-0">"Upfront: "</span>
                <span class="text-neutral-200 font-medium shrink-0">{ upfront_pct_str.clone() }</span>
                <span class="text-neutral-500 shrink-0">" · "</span>
                <span
                    class="text-neutral-200 font-medium inline-block max-w-[12rem] sm:max-w-[16rem] truncate align-bottom"
                    title=move || format_balance(upfront_amount, true)
                >
                    { format_balance(upfront_amount, true) }
                </span>
            </div>
        </div>
    }.into_any();

    let footer = view! {
        <div class="flex items-center justify-between text-xs text-neutral-500">
            <span class="truncate">"Linear vesting after cliff"</span>
            <span class="truncate">
              { blocks_to_human_duration(env.vesting_duration) }
              " · " { blocks_to_str(env.vesting_duration) } " blocks total"
            </span>
        </div>
    }
    .into_any();

    view! {
        <Card padded=true header=header footer=footer class="">
            <div class="space-y-5 min-w-0">
                {/* KPIs */}
                <div class="grid grid-cols-3 gap-4 min-w-0">
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Total Cap"</div>
                        <div class="mt-1 text-base sm:text-lg font-semibold text-neutral-50 truncate"
                             title=move || format_balance(env.total_cap, true)>
                            { format_balance(env.total_cap, true) }
                        </div>
                    </div>
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Cliff"</div>
                        <div class="mt-1 text-base sm:text-lg font-semibold text-neutral-50 truncate"
                             title=move || blocks_to_human_duration(env.cliff)>
                            { blocks_to_human_duration(env.cliff) }
                        </div>
                        <div class="text-xs text-neutral-500 truncate">{ blocks_to_str(env.cliff) } " blocks"</div>
                    </div>
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Vesting Duration"</div>
                        <div class="mt-1 text-base sm:text-lg font-semibold text-neutral-50 truncate"
                             title=move || blocks_to_human_duration(env.vesting_duration)>
                            { blocks_to_human_duration(env.vesting_duration) }
                        </div>
                        <div class="text-xs text-neutral-500 truncate">{ blocks_to_str(env.vesting_duration) } " blocks"</div>
                    </div>
                </div>

                {
                    move || env.unique_beneficiary.as_ref().map(|addr| view!{
                        <div class="rounded-xl border border-white/10 bg-white/5 p-3 sm:p-4">
                            <div class="text-xs uppercase tracking-wider text-neutral-400 mb-2">"Fully allocated to"</div>
                            <div class="flex items-center justify-between gap-3 min-w-0">
                                <div class="flex items-center gap-3 min-w-0">
                                    <div class="h-8 w-8 rounded-full overflow-hidden ring-1 ring-white/10 bg-black/40 shrink-0">
                                        <div inner_html=ss58_identicon_svg(addr, 32) />
                                    </div>
                                    <div class="text-sm text-neutral-300 truncate" title=addr.clone()>{ addr.clone() }</div>
                                </div>
                                <button
                                    class="rounded-full border border-white/10 bg-white/10 hover:bg-white/15 px-3 py-1 text-xs font-medium shrink-0"
                                    title="Copy address"
                                >
                                    "Copy"
                                </button>
                            </div>
                            <div class="mt-2 text-xs text-neutral-500">
                                "Funds from this envelope are sent directly to this address."
                            </div>
                        </div>
                    })
                }

                {
                    (!has_unique).then(move || view!{
                        <div class="min-w-0">
                            <div class="flex items-center justify-between text-xs text-neutral-400">
                                <span class="min-w-0 truncate">"Remaining in envelope"</span>
                                <span class="text-neutral-300 shrink-0">{ format!("{:.0}%", remaining_pct) }</span>
                            </div>
                            <div class="mt-2 h-2 rounded-full bg-white/10 overflow-hidden">
                                <div class="h-full rounded-full bg-emerald-400/70" style=remaining_width></div>
                            </div>

                            <div class="mt-2 grid gap-3 sm:grid-cols-2">
                                {/* Distributed */}
                                <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-3 py-2 min-w-0 text-[13px] sm:text-sm">
                                    <span class="text-neutral-400 shrink-0">"Distributed"</span>
                                    <span class="font-medium text-neutral-100 min-w-0 flex items-baseline gap-1">
                                        <span class="inline-block max-w-[9rem] sm:max-w-[12rem] md:max-w-[14rem] truncate align-bottom"
                                              title=move || format_balance(distributed, true)>
                                            { format_balance(distributed, true) }
                                        </span>
                                        <span class="text-neutral-500 hidden sm:inline whitespace-nowrap shrink-0">
                                            {" · "}{ format!("{:.0}%", distributed_pct) }
                                        </span>
                                    </span>
                                </div>
                                <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-3 py-2 min-w-0 text-[13px] sm:text-sm">
                                    <span class="text-neutral-400 shrink-0">"Remaining"</span>
                                    <span class="font-medium text-neutral-100 min-w-0 flex items-baseline gap-1">
                                        <span class="inline-block max-w-[9rem] sm:max-w-[12rem] md:max-w-[14rem] truncate align-bottom"
                                              title=move || format_balance(remaining, true)>
                                            { format_balance(remaining, true) }
                                        </span>
                                        <span class="text-neutral-500 hidden sm:inline whitespace-nowrap shrink-0">
                                            {" · "}{ format!("{:.0}%", remaining_pct) }
                                        </span>
                                    </span>
                                </div>
                            </div>
                        </div>
                    })
                }
            </div>
        </Card>
    }
}

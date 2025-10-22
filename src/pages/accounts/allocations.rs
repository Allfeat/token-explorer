use leptos::prelude::*;

use crate::{
    components::{fetchable_balance::FetchableData, Card},
    get_allocations_of, get_epoch_duration,
    utils::{blocks_to_human_duration, blocks_to_str, format_balance},
    Allocation,
};

#[component]
pub fn AccountAllocations(id: String) -> impl IntoView {
    let allocs = OnceResource::new(get_allocations_of(id));

    view! {
        <section class="mt-8 sm:mt-10">
            <div class="mb-4 sm:mb-5">
                <h2 class="text-lg sm:text-xl font-semibold text-neutral-50">"Allocations"</h2>
                <p class="text-neutral-400 text-sm">"Rewards currently distributed to this account."</p>
            </div>

            <Suspense fallback=move || view! {
                <div class="grid items-start gap-5 sm:gap-6 lg:grid-cols-2">
                    { (0..2).map(|_| view!{
                        <Card padded=true class="">
                            <div class="space-y-4">
                                <div class="grid grid-cols-3 gap-4">
                                    <div class="h-10 rounded bg-white/10 animate-pulse"></div>
                                    <div class="h-10 rounded bg-white/10 animate-pulse"></div>
                                    <div class="h-10 rounded bg-white/10 animate-pulse"></div>
                                </div>
                                <div class="h-2 rounded bg-white/10 animate-pulse"></div>
                                <div class="grid grid-cols-2 gap-3">
                                    <div class="h-9 rounded bg-white/10 animate-pulse"></div>
                                    <div class="h-9 rounded bg-white/10 animate-pulse"></div>
                                </div>
                            </div>
                        </Card>
                    }).collect::<Vec<_>>() }
                </div>
            }>
                {
                    move || allocs.get().map(|res| match res {
                        Err(_) => view! {
                            <p class="text-neutral-500 text-sm">"No allocation rewards are being distributed."</p>
                        }.into_any(),

                        Ok(list) if list.is_empty() => view! {
                            <p class="text-neutral-500 text-sm">"No allocation rewards are being distributed."</p>
                        }.into_any(),

                        Ok(list) => view! {
                            <div class="grid items-start gap-5 sm:gap-6 lg:grid-cols-2">
                                <For
                                    each=move || list.clone()
                                    key=|a| (a.start, a.total)
                                    children=move |a| {
                                        view! { <AllocationItem allocation=a /> }
                                    }
                                />
                            </div>
                        }.into_any(),
                    })
                }
            </Suspense>
        </section>
    }
}

#[component]
fn AllocationItem(allocation: Allocation) -> impl IntoView {
    let env = allocation.envelope.clone();

    let total = allocation.total;
    let upfront = allocation.upfront;
    let vested_total = allocation.vested_total;
    let released = allocation.released.min(vested_total);

    let progress = if vested_total == 0 {
        0.0
    } else {
        (released as f64 / vested_total as f64).clamp(0.0, 1.0)
    };
    let progress_pct = format!("{:.0}%", progress * 100.0);
    let bar_w = format!("width: {:.2}%;", (progress * 100.0).clamp(0.0, 100.0));

    let per_block = if env.vesting_duration == 0 {
        0.0
    } else {
        vested_total as f64 / env.vesting_duration as f64
    };

    let epoch_blocks = OnceResource::new(get_epoch_duration());

    let header = view! {
        <div class="flex items-center justify-between min-w-0">
            <div class="flex items-center gap-2 min-w-0">
                <span class="text-sm font-medium text-neutral-200 truncate">{ env.name.clone() }</span>
                <span class="text-[10px] px-2 py-0.5 rounded-full border border-white/10 bg-white/5 text-neutral-400 shrink-0">
                    "Envelope"
                </span>
            </div>
        </div>
    }.into_any();

    let footer = view! {
        <div class="flex items-center justify-between text-xs text-neutral-500">
            <span class="truncate">
                "Vesting starts at block " <span class="text-neutral-300">{ allocation.start }</span>
                { " · Cliff " } <span class="text-neutral-300">{ blocks_to_str(env.cliff) }</span> " blocks"
            </span>
            <span class="truncate">
                { blocks_to_human_duration(env.vesting_duration) } " · " { blocks_to_str(env.vesting_duration) } " blocks total"
            </span>
        </div>
    }.into_any();

    view! {
        <Card padded=true header=header footer=footer class="">
            <div class="space-y-5 min-w-0">
                <div class="grid grid-cols-3 gap-4 min-w-0">
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Total"</div>
                        <div class="mt-1 text-lg font-semibold text-neutral-50 truncate"
                             title=move || format_balance(total, true)>
                            { format_balance(total, true) }
                        </div>
                    </div>
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Upfront"</div>
                        <div class="mt-1 text-lg font-semibold text-neutral-50 truncate"
                             title=move || format_balance(upfront, true)>
                            { format_balance(upfront, true) }
                        </div>
                    </div>
                    <div class="min-w-0">
                        <div class="text-[11px] uppercase tracking-wider text-neutral-400">"Vested total"</div>
                        <div class="mt-1 text-lg font-semibold text-neutral-50 truncate"
                             title=move || format_balance(vested_total, true)>
                            { format_balance(vested_total, true) }
                        </div>
                    </div>
                </div>

                <div class="min-w-0">
                    <div class="flex items-center justify-between text-xs text-neutral-400">
                        <span class="min-w-0 truncate">"Released over vesting"</span>
                        <span class="text-neutral-300 shrink-0">{ progress_pct }</span>
                    </div>
                    <div class="mt-2 h-1.5 rounded-full bg-white/10 overflow-hidden">
                        <div class="h-full rounded-full bg-emerald-400/70" style=bar_w></div>
                    </div>
                </div>

                {/* Highlight: Next epoch release */}
                 <FetchableData data=epoch_blocks render=move |n_blocks: u32| {
                    let next_epoch: u128 = ((per_block * n_blocks as f64).floor()) as u128;

                    view! {
                        <div class="rounded-xl border border-white/10 bg-white/5 px-4 py-3 flex items-center justify-between min-w-0">
                            <span class="text-neutral-400 text-sm">"Next epoch release"</span>
                            <span class="text-base sm:text-lg font-semibold text-neutral-50 truncate"
                                  title=move || format_balance(next_epoch, true)>
                                { format_balance(next_epoch, true) }
                            </span>
                        </div>
                    }.into_any()
                } />

            </div>
        </Card>
    }
}

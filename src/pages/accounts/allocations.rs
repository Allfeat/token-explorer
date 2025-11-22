use crate::{
    Allocation,
    components::{Card, fetchable_balance::FetchableData},
    get_allocations_of, get_epoch_duration,
    utils::{blocks_to_human_duration, blocks_to_str, format_balance},
};
use leptos::prelude::*;

#[component]
pub fn AccountAllocations(id: String) -> impl IntoView {
    let allocs = OnceResource::new(get_allocations_of(id));

    view! {
        <section>
             <div class="flex items-center gap-2 mb-4 sm:mb-6">
                <div class="h-1 w-1 rounded-full bg-emerald-500"></div>
                <h2 class="text-sm font-mono uppercase tracking-wider text-neutral-400">
                    "Active Allocations"
                </h2>
            </div>

            <Suspense fallback=move || view! { <AllocationsSkeleton /> }>
                {move || allocs.get().map(|res| match res {
                    Ok(list) if !list.is_empty() => view! {
                        <div class="grid gap-4 sm:gap-6 lg:grid-cols-2">
                            <For
                                each=move || list.clone()
                                key=|a| (a.start, a.total)
                                children=move |a| view! { <AllocationItem allocation=a /> }
                            />
                        </div>
                    }.into_any(),
                    _ => view! {
                         <div class="rounded-2xl border border-dashed border-white/10 bg-white/[0.02] p-6 sm:p-8 text-center">
                            <p class="text-neutral-500 text-sm">"No active vesting schedules found for this account."</p>
                        </div>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn AllocationItem(allocation: Allocation) -> impl IntoView {
    let env = allocation.envelope.clone();
    let total = allocation.total;
    let vested_total = allocation.vested_total;
    let released = allocation.released.min(vested_total);

    let progress = if vested_total == 0 {
        0.0
    } else {
        (released as f64 / vested_total as f64).clamp(0.0, 1.0)
    };
    let progress_pct = format!("{:.1}%", progress * 100.0);
    let bar_w = format!("width: {:.2}%;", (progress * 100.0));

    let per_block = if env.vesting_duration == 0 {
        0.0
    } else {
        vested_total as f64 / env.vesting_duration as f64
    };
    let epoch_blocks = OnceResource::new(get_epoch_duration());

    view! {
        <Card class="h-full flex flex-col">
            <div class="flex flex-col h-full gap-5 sm:gap-6">

                // --- HEADER ---
                <div class="flex items-center justify-between">
                     <div class="flex items-center gap-2 min-w-0">
                        <span class="text-[10px] font-bold uppercase tracking-wider text-emerald-500 shrink-0">"Vesting"</span>
                        <span class="text-neutral-400 text-xs shrink-0">"•"</span>
                        <h3 class="text-sm sm:text-base font-semibold text-white truncate">{ env.name }</h3>
                    </div>
                     // Badge Blocks
                    <div class="text-[10px] font-mono text-neutral-500 bg-white/5 px-1.5 py-0.5 rounded shrink-0 ml-2">
                        {blocks_to_str(env.vesting_duration)} " blks"
                    </div>
                </div>

                // --- MAIN STATS (Grid) ---
                <div class="grid grid-cols-2 gap-x-3 gap-y-4 py-4 border-y border-dashed border-white/10">
                    // Total
                    <div>
                        <div class="text-[10px] uppercase text-neutral-500 mb-1">"Total Alloc."</div>
                        <div class="text-base sm:text-lg font-mono font-bold text-white truncate" title=format_balance(total, true)>
                            { format_balance(total, true) }
                        </div>
                    </div>
                     // Upfront
                    <div>
                        <div class="text-[10px] uppercase text-neutral-500 mb-1">"Upfront Rec."</div>
                         <div class="text-base sm:text-lg font-mono font-medium text-neutral-300 truncate" title=format_balance(allocation.upfront, true)>
                            { format_balance(allocation.upfront, true) }
                        </div>
                    </div>
                </div>

                // --- PROGRESS ---
                <div class="space-y-2">
                     <div class="flex justify-between text-xs">
                        <span class="text-neutral-400">"Vesting Progress"</span>
                        <span class="text-white font-mono">{progress_pct}</span>
                    </div>
                    <div class="h-1.5 w-full rounded-full bg-white/5 overflow-hidden">
                        <div class="h-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" style=bar_w></div>
                    </div>
                     <div class="flex justify-between text-[10px] text-neutral-500 mt-1">
                        <span>"Start: " {allocation.start}</span>
                        <span>"Cliff: " {blocks_to_human_duration(env.cliff)}</span>
                    </div>
                </div>

                // --- NEXT EPOCH PREDICTION ---
                <FetchableData data=epoch_blocks render=move |n_blocks: u32| {
                    let next_epoch: u128 = ((per_block * n_blocks as f64).floor()) as u128;
                    view! {
                         <div class="mt-auto pt-2 sm:pt-4">
                            <div class="rounded bg-black/40 border border-white/5 p-3 flex items-center justify-between">
                                <div class="flex flex-col">
                                    <span class="text-[10px] uppercase text-neutral-500">"Next Epoch Release"</span>
                                    <span class="text-[10px] sm:text-xs text-neutral-600">"Estimated ~24h"</span>
                                </div>
                                <span class="text-sm font-mono font-bold text-emerald-400">
                                    "+"{ format_balance(next_epoch, true) }
                                </span>
                            </div>
                        </div>
                    }.into_any()
                } />
            </div>
        </Card>
    }
}

#[component]
fn AllocationsSkeleton() -> impl IntoView {
    view! {
        <div class="grid gap-4 sm:gap-6 lg:grid-cols-2">
            { (0..2).map(|_| view! {
                <Card class="h-64 animate-pulse">
                    <div class="space-y-4 opacity-50">
                        <div class="h-6 w-1/3 bg-white/10 rounded"></div>
                        <div class="h-px bg-white/10 my-4"></div>
                        <div class="grid grid-cols-2 gap-4">
                            <div class="h-10 bg-white/10 rounded"></div>
                            <div class="h-10 bg-white/10 rounded"></div>
                        </div>
                    </div>
                </Card>
            }).collect::<Vec<_>>() }
        </div>
    }
}

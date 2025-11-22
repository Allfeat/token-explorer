use leptos::prelude::*;

use crate::get_block_number_stream;

#[component]
pub fn BlockStatus() -> impl IntoView {
    let current_block_num = RwSignal::new("Syncing...".to_string());

    let stream_worker = Action::new(move |_: &()| async move {
        match get_block_number_stream().await {
            Ok(stream) => {
                use futures::StreamExt;

                let mut stream = stream.into_inner();
                while let Some(Ok(data)) = stream.next().await {
                    current_block_num.set(data);
                }
            }
            Err(e) => leptos::logging::error!("Stream init error: {:?}", e),
        }
    });

    Effect::new(move |_| {
        stream_worker.dispatch(());
    });

    // --- 2. VIEW ---
    view! {
        <div class="flex items-center gap-3 px-3 py-1.5 rounded-full border border-white/5 bg-white/[0.03]">

            // --- RING ANIMATION ---
            <div class="relative h-5 w-5 flex items-center justify-center">
                {move || {
                    view! {
                        <For
                            each=move || std::iter::once(current_block_num.get())
                            key=|bn| bn.clone()
                            children=move |_| {
                                view! {
                                    <svg class="transform -rotate-90 w-5 h-5" viewBox="0 0 24 24">
                                        <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="transparent" class="text-white/10" />
                                        <circle
                                            cx="12" cy="12" r="10"
                                            stroke="currentColor" stroke-width="3" fill="transparent"
                                            stroke-dasharray="62.83"
                                            stroke-dashoffset="62.83"
                                            stroke-linecap="round"
                                            class="text-emerald-500 animate-block-fill"
                                        />
                                    </svg>
                                }
                            }
                        />
                    }
                }}

                <div class="absolute inset-0 flex items-center justify-center">
                    <div class="h-1.5 w-1.5 rounded-sm bg-emerald-500/50"></div>
                </div>
            </div>

            // --- BLOCK NUMBER TEXT ---
            <div class="flex flex-col leading-none">
                <span class="text-[9px] uppercase tracking-wider text-neutral-500 font-bold">
                    "Current Block"
                </span>
                <div class="flex items-center gap-1 min-h-[1rem]">
                    <span class="text-xs font-mono font-bold text-neutral-200">
                        {move || {
                                if let Ok(num) = current_block_num.get().parse::<u64>() {
                                    format!("#{}", format_number(num))
                                } else {
                                    "Syncing...".to_string()
                                }
                        }}
                    </span>
                </div>
            </div>
        </div>
    }
}

fn format_number(num: u64) -> String {
    let s = num.to_string();
    let mut out = String::with_capacity(s.len() + (s.len() / 3));
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

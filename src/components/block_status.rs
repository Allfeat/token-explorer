use leptos::prelude::*;

#[component]
pub fn BlockStatus() -> impl IntoView {
    // --- 1. DATA SOURCE ---
    #[cfg(not(feature = "ssr"))]
    let block_number = {
        use futures::StreamExt;
        use gloo_net::eventsource::futures::EventSource;
        use send_wrapper::SendWrapper;

        let mut source = SendWrapper::new(
            EventSource::new("/api/sse/blocks").expect("couldn't connect to SSE stream"),
        );

        let stream = source
            .subscribe("message")
            .unwrap()
            .map(|value| match value {
                Ok(value) => {
                    let raw = value.1.data().as_string().unwrap_or_default();
                    raw
                }
                Err(_) => "".to_string(),
            });

        let s = ReadSignal::from_stream_unsync(stream);
        on_cleanup(move || source.take().close());
        s
    };

    #[cfg(feature = "ssr")]
    let (block_number, _) = signal(None::<String>);

    // --- 2. VIEW ---
    view! {
        <div class="flex items-center gap-3 px-3 py-1.5 rounded-full border border-white/5 bg-white/[0.03]">

            // --- RING ANIMATION ---
            <div class="relative h-5 w-5 flex items-center justify-center">
                {move || block_number.get().map(|bn_str| {
                    view! {
                        <For
                            each=move || std::iter::once(bn_str.clone())
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
                })}

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
                        {move || match block_number.get() {
                            Some(bn_str) => {
                                if let Ok(num) = bn_str.parse::<u64>() {
                                    format!("#{}", format_number(num))
                                } else {
                                    bn_str
                                }
                            },
                            None => "Syncing...".to_string()
                        }}
                    </span>
                </div>
            </div>
        </div>
    }
}

// Helper function (identique à avant)
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

use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let current_year = 2025;

    view! {
        <footer class="relative mt-20 border-t border-white/5 bg-[#0b0c0c] py-8">
            <div
                aria-hidden="true"
                class="pointer-events-none absolute inset-x-0 -top-px mx-auto h-24 max-w-2xl bg-[radial-gradient(closest-side,rgba(16,185,129,0.05),transparent_100%)] blur-xl"
            ></div>

            <div class="mx-auto max-w-7xl px-6 flex flex-col md:flex-row items-center justify-between gap-6 text-sm">

                <div class="flex flex-col md:flex-row items-center gap-2 md:gap-6 text-neutral-500">
                    <span>
                        "© " {current_year} " Allfeat"
                    </span>
                    <span class="hidden md:inline text-neutral-800">"|"</span>
                    <span class="flex items-center gap-1.5">
                        "Designed for the Allfeat Economy"
                    </span>
                </div>

                <div class="flex items-center gap-6">
                    <a
                        class="text-neutral-500 transition-colors hover:text-emerald-400 text-xs uppercase tracking-wider font-medium"
                        href="https://github.com/Allfeat/token-explorer/issues/new"
                        rel="noreferrer"
                        target="_blank"
                    >
                        "Report an issue"
                    </a>

                    <div class="flex items-center gap-3 rounded-full border border-white/5 bg-white/[0.02] px-3 py-1.5">
                        <div class="flex items-center gap-2">
                            <span class="relative flex h-1.5 w-1.5">
                                <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-500 opacity-75"></span>
                                <span class="relative inline-flex h-1.5 w-1.5 rounded-full bg-emerald-500"></span>
                            </span>
                            <span class="text-xs font-medium text-neutral-300">"Mainnet"</span>
                        </div>

                        <div class="h-3 w-px bg-white/10"></div>

                        <span class="font-mono text-xs text-neutral-500">
                            "v"{VERSION}
                        </span>
                    </div>
                </div>
            </div>
        </footer>
    }
}

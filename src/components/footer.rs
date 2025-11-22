use chrono::Datelike;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let current_year = chrono::Utc::now().year();

    view! {
        <footer class="relative mt-12 sm:mt-20 border-t border-white/5 bg-[#0b0c0c] py-6 sm:py-8">
            <div
                aria-hidden="true"
                class="pointer-events-none absolute inset-x-0 -top-px mx-auto h-24 max-w-2xl bg-[radial-gradient(closest-side,rgba(16,185,129,0.05),transparent_100%)] blur-xl"
            ></div>

            <div class="mx-auto max-w-7xl px-4 sm:px-6 flex flex-col-reverse md:flex-row items-center justify-between gap-6 sm:gap-8 text-sm">

                <div class="flex flex-col md:flex-row items-center gap-2 text-neutral-500 text-center md:text-left">
                <span>
                        "© " {current_year} " Allfeat"
                    </span>

                    <span class="hidden md:inline text-neutral-800">"|"</span>

                    <span class="text-xs sm:text-sm">
                        "Designed for the Music Economy"
                    </span>
                </div>


                <div class="w-full sm:w-auto flex flex-col sm:flex-row items-center justify-center md:justify-end gap-4 sm:gap-6">
                    <a
                        class="text-neutral-500 transition-colors hover:text-emerald-400 text-xs uppercase tracking-wider font-medium py-2 sm:py-0"
                        href="https://github.com/Allfeat/token-explorer/issues/new"
                        rel="noreferrer"
                        target="_blank"
                    >
                        "Report an issue"
                    </a>

                    // Badge de Versioning style "Status"
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

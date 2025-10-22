use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    view! {
        <footer class="mt-12 border-t border-white/10 bg-[#0b0c0c]">
        <div class="relative">
        <div aria-hidden="true" class="pointer-events-none absolute inset-x-0 -top-10 mx-auto h-40 max-w-6xl bg-[radial-gradient(closest-side,rgba(16,185,129,0.08),transparent_70%)] blur-2xl"></div>
        <div class="max-w-6xl mx-auto px-6 py-10 text-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-6">
        <div class="text-neutral-400">"©" 2025 Allfeat " | " Made with <span class="text-rose-400">" ♥ "</span> by Allfeat</div>

        <div class="flex items-center gap-3">
        </div>
    <div class="text-neutral-600 select-none">
                            {format!("v{}", VERSION)}
                        </div>
        </div>
        </div>
        </footer>
        }
}

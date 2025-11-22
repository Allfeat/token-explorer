use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{components::*, path};

use crate::{
    components::{
        footer::Footer,
        header::Header,
        toast::{ToastProvider, ToastViewport},
    },
    pages::{Account, Allocations, Overview},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>

            <body class="min-h-screen bg-[#151515] text-neutral-200 antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/token-app.css"/>
        <Title text="Allfeat • Economy Explorer"/>

        <Router>
            <div class="relative min-h-dvh flex flex-col bg-[#050505] text-neutral-200 antialiased selection:bg-emerald-500/30 selection:text-emerald-200">

                <div class="fixed inset-0 z-0 pointer-events-none">
                    <div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:40px_40px]"></div>

                    <div class="absolute left-0 right-0 top-[-10%] h-[300px] sm:h-[500px] w-full rounded-full bg-[radial-gradient(circle_farthest-side,rgba(6,182,212,0.08),rgba(255,255,255,0))] blur-[100px]"></div>

                    <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,transparent_0%,#050505_100%)]"></div>
                </div>

                // --- CONTENT WRAPPER ---
                <div class="relative z-10 flex flex-col flex-1">
                    <Header />

                    <ToastProvider>
                        <main class="flex-1 relative">
                            <div class="absolute left-1/2 top-0 -translate-x-1/2 h-full w-px bg-gradient-to-b from-white/5 to-transparent hidden lg:block"></div>

                            <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8 py-8 sm:py-12">
                                <Routes fallback=|| view! { "Page not found" }>
                                    <Route path=path!("") view=Overview/>
                                    <Route path=path!("/accounts/:id") view=Account/>
                                    <Route path=path!("/allocations") view=Allocations/>
                                </Routes>
                            </div>
                            <ToastViewport />
                        </main>
                    </ToastProvider>

                    <Footer />
                </div>
            </div>
        </Router>
    }
}

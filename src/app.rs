use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
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
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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

        <div class="relative min-h-dvh flex flex-col bg-[#151515] text-neutral-200 antialiased">
            <div aria-hidden="true" class="pointer-events-none absolute inset-0 overflow-hidden">
                <div class="absolute -top-24 -right-24 h-80 w-80 bg-gradient-to-br from-fuchsia-500/20 to-cyan-400/20 blur-3xl"></div>
                <div class="absolute -bottom-24 -left-24 h-64 w-64 bg-gradient-to-tr from-fuchsia-500/10 to-cyan-400/10 blur-3xl"></div>
            </div>

            <Header />

            <ToastProvider>
            <main class="relative flex-1">
                <Router>
                    <div class="mx-auto max-w-6xl px-6 sm:px-8 py-8 sm:py-12">
                        <Routes fallback=Overview>
                            <Route path=path!("") view=Overview/>
                            <Route path=path!("/accounts/:id") view=Account/>
                            <Route path=path!("/allocations") view=Allocations/>
                        </Routes>
                    </div>
                </Router>

                <ToastViewport />
            </main>
            </ToastProvider>

            <Footer />
        </div>

    }
}

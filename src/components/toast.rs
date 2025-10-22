use gloo_timers::future::sleep;
use leptos::{prelude::*, task::spawn_local};
use std::{sync::Arc, time::Duration};

// --------- Types ---------
#[derive(Clone)]
pub struct Toast {
    pub id: u32,
    pub message: String,
}

// --------- Contexte Global ---------
#[derive(Clone)]
pub struct ToastContext {
    pub toasts: RwSignal<Vec<Toast>>,
    pub add_toast: Arc<dyn Fn(String) + Send + Sync>,
}

pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>().expect("ToastProvider missing!")
}

// --------- Provider ---------
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toasts = RwSignal::new(Vec::<Toast>::new());
    let counter = RwSignal::new(0u32);

    let add_toast = Arc::new(move |msg: String| {
        let id = counter.get() + 1;
        counter.set(id);
        toasts.update(|ts| {
            ts.push(Toast {
                id,
                message: msg.clone(),
            })
        });
        spawn_local({
            async move {
                sleep(Duration::from_secs(3)).await;
                toasts.update(|ts| ts.retain(|t| t.id != id));
            }
        });
    });

    provide_context(ToastContext { toasts, add_toast });
    view! { {children()} }
}

#[component]
pub fn ToastViewport() -> impl IntoView {
    let ctx = use_toast();
    view! {
        <div class="absolute top-4 right-4 z-40 flex flex-col items-end gap-2 pointer-events-none">
            <For
                each=move || ctx.toasts.get()
                key=|t| t.id
                children=move |t| {
                    view! {
                        <div class="pointer-events-auto px-4 py-2 rounded-full border border-white/20
                                     bg-white/10 text-white backdrop-blur shadow-lg animate-fade-in-right">
                            {t.message.clone()}
                        </div>
                    }
                }
            />
        </div>
    }
}

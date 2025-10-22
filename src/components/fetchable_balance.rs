use leptos::prelude::*;

use crate::{get_balance_of, get_circulating_supply, get_total_issuance, utils::format_balance};

#[component]
pub fn FetchableBalanceFree(address: String) -> impl IntoView {
    let balance = OnceResource::new(get_balance_of(address));

    view! {
        <FetchableData data=balance render=move |value| { view! {
            <span class="text-3xl font-semibold text-neutral-50">
                { format_balance(value.free, true) }
            </span>
        }.into_any() }/>
    }
}

#[component]
pub fn FetchableTotalIssuance() -> impl IntoView {
    let balance = OnceResource::new(get_total_issuance());

    view! {
        <FetchableData data=balance render=move |value| { view! {
            <span class="text-3xl font-semibold text-neutral-50">
                { format_balance(value, true) }
            </span>
        }.into_any() }/>
    }
}

#[component]
pub fn FetchableCirculatingSupply() -> impl IntoView {
    let balance = OnceResource::new(get_circulating_supply());

    view! {
        <FetchableData data=balance render=move |value| { view! {
            <span class="text-3xl font-semibold text-neutral-50">
                { format_balance(value, true) }
            </span>
        }.into_any() }/>
    }
}

#[component]
pub fn FetchableData<T, F>(data: OnceResource<Result<T, ServerFnError>>, render: F) -> impl IntoView
where
    T: 'static + Send + Sync + Clone,
    F: Fn(T) -> AnyView + 'static + Clone + Send + Sync,
{
    view! {
                    <Suspense
                        fallback=move || view! {
                            <span class="inline-block h-[1.2em] w-32 rounded bg-white/10 animate-pulse align-middle"></span>
                        }
                    >
                        {
                            move || data.get().map(|res| match res {
                                Ok(value) => { render(value) },
                                Err(_) => view! {
                                   <span class="inline-block h-[1.2em] w-32 rounded bg-white/10 animate-pulse align-middle"></span>
                                }.into_any(),
                            })
                        }
                    </Suspense>
    }
}

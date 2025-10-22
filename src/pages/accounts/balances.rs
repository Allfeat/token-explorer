use leptos::prelude::*;

use crate::{
    components::{fetchable_balance::FetchableData, Card},
    get_balance_of,
    utils::format_balance,
};

#[component]
pub fn AccountBalances(id: String) -> impl IntoView {
    let balance = OnceResource::new(get_balance_of(id));

    view! {
                    <div class="grid items-start gap-5 sm:gap-6 lg:grid-cols-12">

      <Card
        padded=false
        class="lg:col-span-6 xl:col-span-7 relative overflow-hidden"
        header=view!{
          <div class="flex items-center justify-between p-5 sm:p-6">
            <div class="text-xs sm:text-sm uppercase tracking-wider text-neutral-400">"Transferable"</div>
            <span class="h-2 w-2 rounded-full bg-emerald-400/70"></span>
          </div>
        }.into_any()
      >
        <div aria-hidden="true" class="pointer-events-none absolute inset-0 -z-10">
          <div class="absolute -left-16 -bottom-16 h-44 w-44 rounded-full
                  bg-[radial-gradient(closest-side,rgba(16,185,129,0.18),transparent_70%)]
                  blur-2xl"/>
        </div>

        <div class="p-5 sm:p-6">
          <FetchableData data=balance render=move |value| {
            view! {
              <div class="flex flex-col gap-2">
                <div class="text-3xl sm:text-4xl font-extrabold leading-none text-neutral-50">
                  { format_balance(value.free, true) }
                </div>
                <p class="text-sm text-neutral-400">"Spendable without restrictions."</p>
              </div>
            }.into_any()
          } />
        </div>
      </Card>

      <Card
        padded=false
        class="lg:col-span-3 xl:col-span-3 relative overflow-hidden"
        header=view!{
          <div class="flex items-center justify-between p-5 sm:p-6">
            <div class="text-xs sm:text-sm uppercase tracking-wider text-neutral-400">"Locked"</div>
            <span class="h-2 w-2 rounded-full bg-rose-400/70"></span>
          </div>
        }.into_any()
      >
        <div aria-hidden="true" class="pointer-events-none absolute inset-0 -z-10">
          <div class="absolute -right-14 -bottom-14 h-36 w-36 rounded-full
                  bg-[radial-gradient(closest-side,rgba(244,63,94,0.14),transparent_70%)]
                  blur-2xl"/>
        </div>

        <div class="p-5 sm:p-6">
          <FetchableData data=balance render=move |value| {
            view! {
              <div class="text-2xl sm:text-3xl font-semibold text-neutral-50">
                { format_balance(value.reserved, true) }
              </div>
            }.into_any()
          } />
        </div>
      </Card>

      <Card
        padded=false
        class="lg:col-span-3 xl:col-span-2 relative overflow-hidden"
        header=view!{
          <div class="flex items-center justify-between p-5 sm:p-6">
            <div class="text-xs sm:text-sm uppercase tracking-wider text-neutral-400">"Frozen"</div>
            <span class="h-2 w-2 rounded-full bg-amber-400/80"></span>
          </div>
        }.into_any()
      >
        <div aria-hidden="true" class="pointer-events-none absolute inset-0 -z-10">
          <div class="absolute -right-12 -bottom-12 h-32 w-32 rounded-full
                  bg-[radial-gradient(closest-side,rgba(251,191,36,0.12),transparent_70%)]
                  blur-2xl"/>
        </div>

        <div class="p-5 sm:p-6">
          <FetchableData data=balance render=move |value| {
            view! {
              <div class="text-2xl sm:text-3xl font-semibold text-neutral-50">
                { format_balance(value.frozen, true) }
              </div>
            }.into_any()
          } />
        </div>
      </Card>
    </div>
    }
}

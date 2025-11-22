use super::Card;
use leptos::prelude::*;

#[component]
pub fn SimpleCardMetrics(
    title: &'static str,
    #[prop(optional)] description: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <Card class="h-full">
            <div class="flex flex-col h-full justify-between gap-4">
                <div class="flex flex-col gap-1">
                    <h3 class="text-sm font-medium text-neutral-400 uppercase tracking-wider">
                        {title}
                    </h3>
                    <div class="text-3xl sm:text-4xl font-semibold text-white font-mono tracking-tight mt-1">
                        { children() }
                    </div>
                </div>

                {move || description.map(|desc| view! {
                      <div class="pt-4 mt-auto border-t border-dashed border-white/10">
                        <p class="text-xs text-neutral-500 leading-relaxed">
                            {desc}
                        </p>
                    </div>
                })}
            </div>
        </Card>
    }
}

use crate::{
    components::{
        fetchable_balance::{
            FetchableBalanceFree, FetchableCirculatingSupply, FetchableTotalIssuance,
        },
        simple_card_metrics::SimpleCardMetrics,
    },
    utils::TREASURY_ACCOUNT,
};
use leptos::prelude::*;

#[component]
pub fn TotalIssuance() -> impl IntoView {
    view! {
            <SimpleCardMetrics
                title="Total Supply"
                description="Total capped supply of the network."
            >
              <FetchableTotalIssuance />
            </SimpleCardMetrics>
    }
}

#[component]
pub fn CirculatingSupply() -> impl IntoView {
    view! {
            <SimpleCardMetrics
                title="Circulating Supply"
                description="Total supply in circulation on the network."
            >
              <FetchableCirculatingSupply />
            </SimpleCardMetrics>
    }
}

#[component]
pub fn TreasuryBalance() -> impl IntoView {
    view! {
            <SimpleCardMetrics
                title="Treasury Funds"
                description="Available funds in the treasury of the network."
            >
              <FetchableBalanceFree address=TREASURY_ACCOUNT.into() />
            </SimpleCardMetrics>
    }
}

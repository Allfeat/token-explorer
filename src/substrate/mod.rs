use subxt::{OnlineClient, SubstrateConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(
    runtime_metadata_path = "./artifacts/allfeat_metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod allfeat {}

pub type AllfeatClient = OnlineClient<SubstrateConfig>;

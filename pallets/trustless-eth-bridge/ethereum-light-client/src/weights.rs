use frame_support::weights::Weight;

pub trait WeightInfo {
    fn import_header() -> Weight;
    fn import_header_not_new_finalized_with_max_prune() -> Weight;
    fn import_header_new_finalized_with_single_prune() -> Weight;
    fn import_header_not_new_finalized_with_single_prune() -> Weight;
    fn register_network() -> Weight;
    fn update_difficulty_config() -> Weight;
}

impl WeightInfo for () {
    fn import_header() -> Weight {
        0
    }
    fn import_header_not_new_finalized_with_max_prune() -> Weight {
        0
    }
    fn import_header_new_finalized_with_single_prune() -> Weight {
        0
    }
    fn import_header_not_new_finalized_with_single_prune() -> Weight {
        0
    }
    fn register_network() -> Weight {
        0
    }
    fn update_difficulty_config() -> Weight {
        0
    }
}

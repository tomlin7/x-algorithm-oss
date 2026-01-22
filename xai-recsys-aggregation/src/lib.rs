pub mod aggregation {
    use xai_uas_thrift::user_action_sequence::{
        AggregatedUserAction as ThriftAggregatedUserAction,
        UserAction as ThriftUserAction,
    };

    pub trait UserActionAggregator: Send + Sync {
        fn name(&self) -> String { "dummy".to_string() }
        fn run(&self, _: &[ThriftUserAction], _: i64, _: i64) -> Vec<ThriftAggregatedUserAction> { vec![] }
    }
    pub struct DefaultAggregator;
    impl UserActionAggregator for DefaultAggregator {}
}
pub mod filters {
    use xai_uas_thrift::user_action_sequence::{
        AggregatedUserAction as ThriftAggregatedUserAction,
        UserAction as ThriftUserAction,
    };

    pub trait AggregatedActionFilter: Send + Sync {
        fn run(&self, actions: Vec<ThriftAggregatedUserAction>) -> Vec<ThriftAggregatedUserAction> { actions }
    }
    pub trait DenseAggregatedActionFilter: Send + Sync {
         fn run(&self, actions: Vec<ThriftAggregatedUserAction>) -> Vec<ThriftAggregatedUserAction> { actions }
    }
    pub trait KeepOriginalUserActionFilter: Send + Sync {
         fn run(&self, actions: Vec<ThriftUserAction>) -> Vec<ThriftUserAction> { actions }
    }
    pub trait UserActionFilter: Send + Sync {
         fn run(&self, actions: Vec<ThriftUserAction>) -> Vec<ThriftUserAction> { actions }
    }
    
    // Implement dummy structs for new() calls in hydrator
    pub struct DummyDenseAggregatedActionFilter;
    impl DenseAggregatedActionFilter for DummyDenseAggregatedActionFilter {}
    impl DummyDenseAggregatedActionFilter { pub fn new() -> Self { Self } }
    impl AggregatedActionFilter for DummyDenseAggregatedActionFilter {}

    pub struct DummyKeepOriginalUserActionFilter;
    impl KeepOriginalUserActionFilter for DummyKeepOriginalUserActionFilter {}
    impl DummyKeepOriginalUserActionFilter { pub fn new() -> Self { Self } }
    impl UserActionFilter for DummyKeepOriginalUserActionFilter {}
}

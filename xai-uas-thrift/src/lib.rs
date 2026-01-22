pub mod convert {
    pub fn thrift_to_proto_aggregated_user_action(_: ()) -> () { () }
}
pub mod user_action_sequence {
    #[derive(Clone, Debug, Default)]
    pub struct AggregatedUserAction {
        pub impressed_time_ms: Option<i64>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct UserAction {
        // Dummy fields
        pub action: Option<i32>, // Dummy
    }

    #[derive(Clone, Debug, Default)]
    pub struct UserActionSequenceMeta {
        pub last_modified_epoch_ms: Option<i64>,
        pub last_kafka_publish_epoch_ms: Option<i64>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct UserActionSequence {
        pub user_actions: Option<Vec<UserAction>>,
        pub metadata: Option<UserActionSequenceMeta>,
    }
}

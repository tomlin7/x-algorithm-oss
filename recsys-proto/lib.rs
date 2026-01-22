
#[derive(Clone, Debug, Default)]
pub struct TweetInfo {
    pub tweet_id: u64,
    pub author_id: u64,
    pub in_reply_to_tweet_id: u64,
}

#[derive(Clone, Debug, Default)]
pub struct PredictNextActionsResponse {
    pub distribution_sets: Vec<DistributionSet>,
}

#[derive(Clone, Debug, Default)]
pub struct CandidateDistribution {
    pub candidate: Option<TweetInfo>,
    pub top_log_probs: Vec<f32>,
    pub continuous_actions_values: Vec<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct DistributionSet {
    pub candidate_distributions: Vec<CandidateDistribution>,
}

#[derive(Clone, Copy, Debug)]
pub enum ActionName {
    ServerTweetFav,
    ServerTweetReply,
    ServerTweetRetweet,
    ClientTweetPhotoExpand,
    ClientTweetClick,
    ClientTweetProfileClick,
    ClientTweetShare,
    ClientTweetShareViaDm,
    ClientTweetShareViaCopyLink,
    ServerTweetQuote,
    ClientTweetQuotedClick,
    ServerTweetUserFollow,
    ClientTweetNotInterested,
    ClientTweetUserBlock,
    ClientTweetUserMute,
    ClientTweetUserReport,
    ClientTweetVideoView50,
    ClientTweetClickProfile,
    ClientTweetVideoQualityView,
    ClientTweetClickSendViaDirectMessage,
    ClientTweetRecapDwelled,
    ClientQuotedTweetClick,
    ClientTweetFollowAuthor,
    ClientTweetNotInterestedIn,
    ClientTweetBlockAuthor,
    ClientTweetMuteAuthor,
    ClientTweetReport,
}
#[derive(Clone, Copy, Debug)]
pub enum ContinuousActionName {
    DwellTime,
}

#[derive(Clone, Debug, Default)]
pub struct AggregatedUserActionList {
    pub aggregated_user_actions: Vec<xai_uas_thrift::user_action_sequence::AggregatedUserAction>,
    pub aggregation_provider: String,
    pub aggregation_time_ms: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Mask {
    pub mask_type: i32,
    pub mask: Vec<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum MaskType {
    #[default]
    NewEvent = 0,
}

#[derive(Clone, Debug, Default)]
pub struct UserActionSequenceMeta {
    pub length: u64,
    pub first_sequence_time: u64,
    pub last_sequence_time: u64,
    pub last_modified_epoch_ms: u64,
    pub previous_kafka_publish_epoch_ms: u64,
}

#[derive(Clone, Debug, Default)]
pub struct UserActionSequenceDataContainer {
    pub data: Option<user_action_sequence_data_container::Data>,
}

pub mod user_action_sequence_data_container {
    #[derive(Clone, Debug)]
    pub enum Data {
        OrderedAggregatedUserActionsList(crate::AggregatedUserActionList),
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserActionSequence {
    pub user_id: u64,
    pub metadata: Option<UserActionSequenceMeta>,
    pub user_actions_data: Option<UserActionSequenceDataContainer>,
    pub masks: Vec<Mask>,
}

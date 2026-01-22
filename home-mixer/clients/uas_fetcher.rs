pub struct UserActionSequenceFetcher;
pub trait UserActionSequenceOps {}
use xai_uas_thrift::user_action_sequence::UserActionSequence as ThriftUserActionSequence;

impl UserActionSequenceFetcher {
    pub fn new() -> Result<Self, ()> { Ok(Self) }
    pub async fn get_by_user_id(&self, _: i64) -> Result<ThriftUserActionSequence, String> { Ok(ThriftUserActionSequence::default()) }
}

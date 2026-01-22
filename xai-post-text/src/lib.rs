pub struct MatchTweetGroup;
impl MatchTweetGroup {
    pub fn new(_: UserMutes) -> Self { Self }
    pub fn matches(&self, _: &TokenSequence) -> bool { false }
}
pub struct TokenSequence;
pub struct TweetTokenizer;
impl TweetTokenizer {
    pub fn new() -> Self { Self }
    pub fn tokenize(&self, _: &str) -> TokenSequence { TokenSequence }
}
pub struct UserMutes;
impl UserMutes {
    pub fn new(_: Vec<TokenSequence>) -> Self { Self }
}

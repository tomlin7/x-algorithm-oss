pub struct BloomFilter;
impl BloomFilter {
    pub fn from_entry<T>(_: T) -> Self { Self }
    pub fn may_contain(&self, _: i64) -> bool { false }
}

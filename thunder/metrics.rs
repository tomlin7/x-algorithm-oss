use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub struct HistogramVec;
impl HistogramVec {
    pub fn with_label_values(&self, _labels: &[&str]) -> Histogram {
        Histogram
    }
    pub fn observe(&self, _v: f64) {}
    pub fn set(&self, _v: f64) {}
}

#[derive(Clone, Copy)]
pub struct Histogram;
impl Histogram {
    pub fn observe(&self, _v: f64) {}
    pub fn set(&self, _v: f64) {}
}

#[derive(Clone, Copy)]
pub struct Counter;
impl Counter {
    pub fn inc(&self) {}
    pub fn dec(&self) {}
}

lazy_static! {
    pub static ref GET_IN_NETWORK_POSTS_COUNT: Histogram = Histogram;
    pub static ref GET_IN_NETWORK_POSTS_DURATION: Histogram = Histogram;
    pub static ref GET_IN_NETWORK_POSTS_DURATION_WITHOUT_STRATO: Histogram = Histogram;
    pub static ref GET_IN_NETWORK_POSTS_EXCLUDED_SIZE: Histogram = Histogram;
    pub static ref GET_IN_NETWORK_POSTS_FOLLOWING_SIZE: Histogram = Histogram;
    pub static ref GET_IN_NETWORK_POSTS_FOUND_FRESHNESS_SECONDS: HistogramVec = HistogramVec;
    pub static ref GET_IN_NETWORK_POSTS_FOUND_POSTS_PER_AUTHOR: HistogramVec = HistogramVec;
    pub static ref GET_IN_NETWORK_POSTS_FOUND_REPLY_RATIO: HistogramVec = HistogramVec;
    pub static ref GET_IN_NETWORK_POSTS_FOUND_TIME_RANGE_SECONDS: HistogramVec = HistogramVec;
    pub static ref GET_IN_NETWORK_POSTS_FOUND_UNIQUE_AUTHORS: HistogramVec = HistogramVec;
    pub static ref IN_FLIGHT_REQUESTS: Counter = Counter;
    pub static ref REJECTED_REQUESTS: Counter = Counter;
    pub static ref GET_IN_NETWORK_POSTS_MAX_RESULTS: Histogram = Histogram;
    
    // PostStore metrics
    pub static ref POST_STORE_DELETED_POSTS: Histogram = Histogram;
    pub static ref POST_STORE_DELETED_POSTS_FILTERED: Counter = Counter;
    pub static ref POST_STORE_ENTITY_COUNT: HistogramVec = HistogramVec;
    pub static ref POST_STORE_POSTS_RETURNED: Histogram = Histogram;
    pub static ref POST_STORE_POSTS_RETURNED_RATIO: Histogram = Histogram;
    pub static ref POST_STORE_REQUEST_TIMEOUTS: Counter = Counter;
    pub static ref POST_STORE_REQUESTS: Counter = Counter;
    pub static ref POST_STORE_TOTAL_POSTS: Histogram = Histogram;
    pub static ref POST_STORE_USER_COUNT: Histogram = Histogram;
}

pub struct Timer;
impl Timer {
    pub fn new(_: Histogram) -> Self {
        Self
    }
}

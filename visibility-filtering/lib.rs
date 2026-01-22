pub mod models {
    #[derive(Clone, Debug, Default)]
    pub struct FilteredReason {}
    
    impl From<FilteredReason> for i32 {
        fn from(_: FilteredReason) -> Self {
            0
        }
    }
}

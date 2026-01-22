pub type StratoResult<T> = Result<T, ()>;
pub struct StratoValue<T> {
    pub v: Option<T>,
    pub _marker: std::marker::PhantomData<T>,
}
pub fn decode<T, U>(_: &StratoValue<U>) -> StratoResult<StratoValue<T>> { 
    Ok(StratoValue { v: None, _marker: std::marker::PhantomData }) 
}

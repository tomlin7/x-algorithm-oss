pub fn duration_since_creation_opt(id: i64) -> Option<std::time::Duration> {
    let id = id as u64;
    let timestamp = (id >> 22) + 1288834974657;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?.as_millis() as u64;
    
    if now >= timestamp {
        Some(std::time::Duration::from_millis(now - timestamp))
    } else {
        Some(std::time::Duration::from_millis(0)) // Future
    }
}


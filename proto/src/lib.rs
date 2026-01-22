pub mod home_mixer {
    tonic::include_proto!("xai_home_mixer_proto");
}
// Re-export types for convenient access
pub use home_mixer::*;

// Add the file descriptor set for reflection (needs compilation to work, 
// using a dummy empty set for now or needs proper setup, skipping for MVP buildability unless needed)
pub const FILE_DESCRIPTOR_SET: &[u8] = &[];

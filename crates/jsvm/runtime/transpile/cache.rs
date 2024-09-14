use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct EmitMetadata {
    pub source_hash: u64,
    pub target_hash: u64,
}

struct EmitCache {}

pub struct JsvmTranspile {}

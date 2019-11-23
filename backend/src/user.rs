use super::proto;
use exonum::crypto::{Hash, PublicKey};

/// Stores information about a participant
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::User", serde_pb_convert)]
pub struct User {
    /// key
    pub key: PublicKey,
}

impl User {
    /// Creates new user
    pub fn new(
        &key: &PublicKey,
    ) -> Self {
        Self {
            key
        }
    }
}

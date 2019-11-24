use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};
use exonum_merkledb::{ListProof, MapProof};

use super::{schema::Schema, SERVICE_ID};
use crate::classes::*;

/// Describes the query parameters for the `get_user` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct UserQuery {
    /// Public key of the queried user.
    pub pub_key: PublicKey,
}

/// Describes the query parameters for the `get_cert` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CertQuery {
    /// Public key of the queried user.
    pub pub_key: PublicKey,

    /// Public key of the queried user.
    pub course_name: String,
}

/// Proof of existence for specific user.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProof {
    /// Proof of the whole database table.
    pub to_table: MapProof<Hash, Hash>,
    /// Proof of the specific user in this table.
    pub to_user: MapProof<PublicKey, User>,
}

/// User history.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserHistory {
    /// Proof of the list of transaction hashes.
    pub proof: ListProof<Hash>,
    /// List of above transactions.
    pub transactions: Vec<TransactionMessage>,
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    fn user_exist(state: &ServiceApiState, query: UserQuery,) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);
        let user = schema.user(&query.pub_key);

        let answer = match user {
            None => false,
            Some(_i) => true,
        };

        Ok(answer)
    }

    fn get_cert(state: &ServiceApiState, query: CertQuery,) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let &pub_key = &query.pub_key;
        let course_name = &query.course_name;

        // TODO LOGIC

        Ok(true)
    }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            // v1/educator/user_exist?pub_key={id}
            .endpoint("v1/educator/user_exist", Self::user_exist)
            // v1/educator/user_exist?pub_key={id}&course_name={name}
            .endpoint("v1/educator/get_certs", Self::get_cert);
    }
}

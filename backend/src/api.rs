use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};
use exonum_merkledb::{ListProof, MapProof};

use super::{schema::Schema, SERVICE_ID};
use crate::user::User;

/// Get first user key
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GetFirstQuery {}

/// Describes the query parameters for the `get_user` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct UserQuery {
    /// Public key of the queried user.
    pub pub_key: PublicKey,
}

/// Describes the query parameters for the `get_cert` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

/// User information.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// Proof of the last block.
    pub block_proof: BlockProof,
    /// Proof of the appropriate user.
    pub user_proof: UserProof,
    /// History of the appropriate user.
    pub user_history: Option<UserHistory>,
}

/// User information.
#[derive(Debug, Serialize, Deserialize)]
pub struct CertInfo {
    /// Proof of the last block.
    pub block_proof: BlockProof,
    /// Proof of the appropriate user.
    pub user_proof: UserProof,
    /// History of the appropriate user.
    pub user_history: Option<UserHistory>,
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Endpoint for getting a single user.
    // fn user_info(
    //     state: &ServiceApiState,
    //     query: UserQuery,
    // ) -> api::Result<UserInfo> {
    //     let snapshot = state.snapshot();
    //     let general_schema = blockchain::Schema::new(&snapshot);
    //     let currency_schema = Schema::new(&snapshot);

    //     let max_height = general_schema.block_hashes_by_height().len() - 1;

    //     let block_proof = general_schema
    //         .block_and_precommits(Height(max_height))
    //         .unwrap();

    //     let to_table: MapProof<Hash, Hash> =
    //         general_schema.get_proof_to_service_table(SERVICE_ID, 0);

    //     let to_user: MapProof<PublicKey, User> =
    //         currency_schema.users().get_proof(query.pub_key);

    //     let user_proof = UserProof {
    //         to_table,
    //         to_user,
    //     };

    //     let user = currency_schema.user(&query.pub_key);

    //     let explorer = BlockchainExplorer::new(state.blockchain());

    //     let user_history = user.map(|_| {
    //         let history = currency_schema.user_history(&query.pub_key);
    //         let proof = history.get_range_proof(0..history.len());

    //         let transactions = history
    //             .iter()
    //             .map(|record| explorer.transaction_without_proof(&record).unwrap())
    //             .collect::<Vec<_>>();

    //         UserHistory {
    //             proof,
    //             transactions,
    //         }
    //     });

    //     Ok(UserInfo {
    //         block_proof,
    //         user_proof,
    //         user_history,
    //     })
    // }

    fn user_exist(state: &ServiceApiState, query: UserQuery,) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);
        let user = schema.user(&query.pub_key);

        let answer = match user {
            None => false,
            Some(i) => true,
        };

        Ok(answer)
    }

    fn get_cert(state: &ServiceApiState, query: CertQuery,) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let &pub_key = &query.pub_key;
        let &course_name = &query.course_name;

        // TODO LOGIC

        Ok(answer)
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

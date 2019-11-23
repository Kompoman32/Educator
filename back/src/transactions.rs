#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::{proto, schema::Schema, SERVICE_ID};

/// Error codes emitted by pipes transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Participant already exists.
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Participant already exists")]
    ParticipantAlreadyExists = 0,
    
    /// Participant already removed.
    ///
    /// Can be emitted by `Remove`.
    #[fail(display = "Participant already removed")]
    ParticipantAlreadyRemoved = 1,

    /// Can't find participant by key.
    ///
    /// Can be emitted by `Remove`.
    #[fail(display = "Can't find participant by key")]
    ParticipantNotFound = 2,

    /// Participant already bought a phone.
    ///
    /// Can be emitted by `Buy`.
    #[fail(display = "Participant already bought a phone")]
    ParticipantAlreadyBought = 3,

    /// Participant is not first.
    ///
    /// Can be emitted by `Buy`.
    #[fail(display = "Participant is not first")]
    ParticipantIsNotFirst = 4
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

/// Create participant.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Add_User")]
pub struct Add_User {
    /// `PublicKey` of participant.
    pub key: PublicKey
}

///
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Add_Class")]
pub struct Add_Class {
    ///
    pub student_key: PublicKey,
    ///
    pub class_name: String
}

// /// Buy a phone.
// #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
// #[exonum(pb = "proto::Buy")]
// pub struct Buy {
//     /// `PublicKey` of participant.
//     pub key: PublicKey,
// }

// /// Remove from queue.
// #[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
// #[exonum(pb = "proto::Remove")]
// pub struct Remove {
//     /// `PublicKey` of participant.
//     pub key: PublicKey,
// }

/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum UserTransactions {
    /// Add tx.
    AddUser(Add_User),
    ///
    AddClass(Add_Class),
    // /// Buy tx.
    // Buy(Buy),
    // /// Remove tx.
    // Remove(Remove)
}

impl Add_User {
    #[doc(hidden)]
    pub fn sign(
        &key: &PublicKey,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key }, SERVICE_ID, *pk, sk)
    }
}

impl Add_Class {
    #[doc(hidden)]
    pub fn sign(
        &key: &PublicKey,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Add_User { key }, SERVICE_ID, *pk, sk)
    }
}

// impl Buy {
//     #[doc(hidden)]
//     pub fn sign(
//         pk: &PublicKey,
//         &key: &PublicKey,
//         sk: &SecretKey,
//     ) -> Signed<RawTransaction> {
//         Message::sign_transaction(Self { key }, SERVICE_ID, *pk, sk)
//     }
// }

// impl Remove {
//     #[doc(hidden)]
//     pub fn sign(
//         pk: &PublicKey,
//         &key: &PublicKey,
//         sk: &SecretKey,
//     ) -> Signed<RawTransaction> {
//         Message::sign_transaction(Self { key }, SERVICE_ID, *pk, sk)
//     }
// }

impl Transaction for Add_User {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let key = &self.key;

        if schema.user(key).is_none() {
            schema.add_user(key);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}

impl Transaction for Add_Class {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let key = &self.student_key;

        if schema.user(key).is_none() {
            schema.add_user(key);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}

// impl Transaction for Buy {
//     fn execute(&self, context: TransactionContext) -> ExecutionResult {
//         let hash = context.tx_hash();
//         let mut schema = Schema::new(context.fork());
//         let key = &self.key;

//         if let Some(participant) = schema.participant(key) {
//             if participant.have_bought {
//                 Err(Error::ParticipantAlreadyBought)?
//             }
            
//             let first = schema.first_participant().unwrap();
//             if !first.key.eq(&participant.key) {
//                 Err(Error::ParticipantIsNotFirst)?
//             }

//             schema.participant_have_bought(participant, &hash);
//             Ok(())
//         } else {
//             Err(Error::ParticipantNotFound)?
//         }
//     }
// }

// impl Transaction for Remove {
//     fn execute(&self, context: TransactionContext) -> ExecutionResult {
//         let hash = context.tx_hash();
//         let mut schema = Schema::new(context.fork());
//         let key = &self.key;

//         if let Some(participant) = schema.participant(key) {

//             if participant.removed {
//                 Err(Error::ParticipantAlreadyRemoved)?
//             }

//             schema.remove_participant(participant, &hash);
//             Ok(())
//         } else {
//             Err(Error::ParticipantNotFound)?
//         }
//     }
// }

#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::{proto, schema::Schema, SERVICE_ID};
use crate::hardStorage::*;
use crate::smartContract::can_get_cert;

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
#[exonum(pb = "proto::User")]
pub struct User {
    /// `PublicKey` of participant.
    pub key: PublicKey
}

///
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Class")]
pub struct Class {
    ///
    pub student_key: PublicKey,
    ///
    pub class_name: String
}

///
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Task")]
pub struct Task {
    ///
    pub student_key: PublicKey,
    ///
    pub task_name: String
}

/// Create participant.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Cert")]
pub struct Cert {
    /// `PublicKey` of participant.
    pub student_key: PublicKey,
    /// 
    pub course_name: String,
}





/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum UserTransactions {
    /// Add tx.
    AddUser(User),
    ///
    AddClass(Class),
    ///
    AddTask(Task),
    ///
    GetCert(Cert),
}

impl Transaction for User {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(context.fork());

        let key = &self.key;
        let author = context.author();

        if !can_add_user(&author) {
            return Err(Error::ParticipantAlreadyExists)?
        }
        if schema.user(key).is_none() {
            schema.add_user(key);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}

impl Transaction for Class {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(context.fork());

        let key = &self.student_key;
        let class_name = self.class_name.chars().collect();
        let author = context.author();

        if can_add_class(&author) {
            return Err(Error::ParticipantAlreadyExists)?
        }

        if schema.user(key).is_none() {
            Err(Error::ParticipantAlreadyExists)?
        } else {
            if schema.class(key, &class_name).is_none(){
                Err(Error::ParticipantAlreadyExists)?
            } else {
                schema.add_class(key, &class_name);
                Ok(())
            }
        }
    }
}

impl Transaction for Task {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(context.fork());

        let key = &self.student_key;
        let task_name = self.task_name.chars().collect();
        let author = context.author();

        if can_add_task(&author) {
            return Err(Error::ParticipantAlreadyExists)?
        }

        if schema.user(key).is_none() {
            Err(Error::ParticipantAlreadyExists)?
        } else {
            if schema.task(key, &task_name).is_none(){
                Err(Error::ParticipantAlreadyExists)?
            } else {
                schema.add_task(key, &task_name);
                Ok(())
            }
        }
    }
}

impl Transaction for Cert {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(context.fork());

        let key = &self.student_key;
        let course_name = self.course_name.chars().collect();
        let author = context.author();

        if can_add_cert(&author) {
            return Err(Error::ParticipantAlreadyExists)?
        }

        if schema.user(key).is_none() {
            Err(Error::ParticipantAlreadyExists)?
        } else {
            if can_get_cert(key, &course_name) {
                schema.add_cert(key, &course_name);
                Ok(())
            } else {
                Err(Error::ParticipantAlreadyExists)?
            }
        }
    }
}

/// FOR TESTS

impl User {
    #[doc(hidden)]
    pub fn sign(
        &key: &PublicKey,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key }, SERVICE_ID, *pk, sk)
    }
}

impl Class {
    #[doc(hidden)]
    pub fn sign(
        &student_key: &PublicKey,
        class_name: String,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { student_key, class_name }, SERVICE_ID, *pk, sk)
    }
}

impl Task {
    #[doc(hidden)]
    pub fn sign(
        &student_key: &PublicKey,
        task_name: String,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { student_key, task_name }, SERVICE_ID, *pk, sk)
    }
}

impl Cert {
    #[doc(hidden)]
    pub fn sign(
        &student_key: &PublicKey,
        course_name: String,
        pk: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { student_key, course_name }, SERVICE_ID, *pk, sk)
    }
}
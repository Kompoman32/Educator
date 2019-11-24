#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::{proto, schema::Schema, SERVICE_ID};
use crate::hard_storage::*;
use crate::smart_contract::can_get_cert;

/// Error codes emitted by pipes transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Participant already exists.
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Пользователь уже существует")]
    UserAlreadyExist = 0,

    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Пользователь не существует")]
    UserIsNotExist = 1,

    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Отсутсвуют права на добавление пользователя")]
    NoAccessToAddUser = 2,
    
    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Отсутсвуют права на добавление отметки посещения занятия")]
    NoAccessToAddClass = 3,
    
    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Отсутсвуют права на добавление отметки выполения задания")]
    NoAccessToAddTask = 4,
    
    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Отсутсвуют права на добавление сертификата")]
    NoAccessToAddCert = 5,
    
    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Запись о посещении занятия уже сущесвует")]
    ClassRecordAlreadyExist = 6,
    
    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Запись о выполнении задания уже сущесвует")]
    TaskRecordAlreadyExist = 7,

    /// 
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Выполнены не все условия для получения сертификата")]
    CertConditionsIsNotSatisfied = 8,
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
    pub key: PublicKey,
    /// name
    pub name: String
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
        let name = self.name.chars().collect();;
        let author = context.author();

        if !can_add_user(&author) {
            return Err(Error::NoAccessToAddUser)?
        }

        if schema.user(key).is_none() {
            schema.add_user(key, &name);

            Ok(())
        } else {
            Err(Error::UserAlreadyExist)?
        }
    }
}

impl Transaction for Class {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(context.fork());

        let key = &self.student_key;
        let class_name = self.class_name.chars().collect();
        let author = context.author();

        if !can_add_class(&author) {
            return Err(Error::NoAccessToAddClass)?
        }

        if schema.user(key).is_none() {
            Err(Error::UserIsNotExist)?
        } else {
            if schema.class(key, &class_name).is_none(){
                schema.add_class(key, &class_name);
                Ok(())
            } else {
                Err(Error::ClassRecordAlreadyExist)?
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

        if !can_add_task(&author) {
            return Err(Error::NoAccessToAddTask)?
        }

        if schema.user(key).is_none() {
            Err(Error::UserIsNotExist)?
        } else {
            if schema.task(key, &task_name).is_none(){
                schema.add_task(key, &task_name);
                Ok(())
            } else {
                Err(Error::TaskRecordAlreadyExist)?
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

        if !can_add_cert(&author) {
            return Err(Error::NoAccessToAddCert)?
        }

        if schema.user(key).is_none() {
            Err(Error::UserIsNotExist)?
        } else {
            if can_get_cert(key, &course_name) {
                if schema.cert(key, &course_name).is_none() {
                    schema.add_cert(key, &course_name);
                }
                Ok(())
            } else {
                Err(Error::CertConditionsIsNotSatisfied)?
            }
        }
    }
}

/// FOR TESTS

impl User {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &key: &PublicKey,
        name: String,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key, name }, SERVICE_ID, *pk, sk)
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
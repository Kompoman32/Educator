use super::proto;
use exonum::crypto::{Hash, PublicKey};

/// Stores information about a user
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::User", serde_pb_convert)]
pub struct User {
    /// key
    pub key: PublicKey,
    /// name
    pub name: String,
}

impl User {
    /// Creates new user
    pub fn new(
        &key: &PublicKey,
        name: String,
    ) -> Self {
        Self {
            key,
            name,
        }
    }
}

/// Stores information about a Class
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Class", serde_pb_convert)]
pub struct Class {
    /// key
    pub student_key: PublicKey,
    ///
    pub class_name: String,
}

impl Class {
    /// Creates new class
    pub fn new(
        &student_key: &PublicKey,
        class_name: String,
    ) -> Self {
        Self {
            student_key,
            class_name
        }
    }
}

/// Stores information about a Task
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Task", serde_pb_convert)]
pub struct Task {
    /// key
    pub student_key: PublicKey,
    ///
    pub task_name: String,
}

impl Task {
    /// Creates new task
    pub fn new(
        &student_key: &PublicKey,
        task_name: String,
    ) -> Self {
        Self {
            student_key,
            task_name
        }
    }
}

/// Stores information about a Cert
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Cert", serde_pb_convert)]
pub struct Cert {
    /// key
    pub student_key: PublicKey,
    ///
    pub course_name: String,
}

impl Cert {
    /// Creates new cert
    pub fn new(
        &student_key: &PublicKey,
        course_name: String,
    ) -> Self {
        Self {
            student_key,
            course_name
        }
    }
}


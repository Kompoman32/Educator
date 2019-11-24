//! Educator database schema.
use crate::classes::*;
use exonum::crypto::{Hash, PublicKey};
use exonum_merkledb::{IndexAccess, ObjectHash, ProofMapIndex};

/// User table name
pub const USER_TABLE: &str = "educator.user";
/// Class table name
pub const CLASS_TABLE: &str = "educator.class";
/// Task table name
pub const TASK_TABLE: &str = "educator.task";
/// Cert table name
pub const CERT_TABLE: &str = "educator.cert";

/// Database schema.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
where
    T: IndexAccess,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    /// Returns `ProofMapIndex` with users.
    pub fn users(&self) -> ProofMapIndex<T, PublicKey, User> {
        ProofMapIndex::new(USER_TABLE, self.view.clone())
    }

    /// Returns user for the given public key.
    pub fn user(&self, pub_key: &PublicKey) -> Option<User> {
        self.users().get(pub_key)
    }

    /// Create new user and append first record to its history.
    pub fn add_user(
        &mut self,
        key: &PublicKey,
        name: &String,
    ) {
        let created_user = {
            User::new(
                key,
                name.clone()
            )
        };
        self.users().put(key, created_user);
    }

    /// Returns the state hash of service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.users().object_hash(), self.classes().object_hash()]
    }




    ///
    fn convert_to_class_key(&self, key: &PublicKey, class_name: &String) -> String {
        key.to_string() + &class_name.clone()
    }

    /// Returns `ProofMapIndex` with users.
    pub fn classes(&self) -> ProofMapIndex<T, String, Class> {
        ProofMapIndex::new(CLASS_TABLE, self.view.clone())
    }

    /// Returns user for the given public key.
    pub fn class(&self, key: &PublicKey, class_name: &String) -> Option<Class> {
        self.classes().get(&self.convert_to_class_key(key, class_name))
    }

    /// Create new user and append first record to its history.
    pub fn add_class(
        &mut self,
        student_key: &PublicKey,
        class_name: &String,
    ) {
        let created_class = {
            Class::new(
                student_key,
                class_name.clone()
            )
        };
        self.classes().put(&self.convert_to_class_key(student_key, class_name), created_class);
    }



    ///
    fn convert_to_task_key(&self, key: &PublicKey, task_name: &String) -> String {
        key.to_string() + &task_name.clone()
    }

    /// Returns `ProofMapIndex` with users.
    pub fn taskes(&self) -> ProofMapIndex<T, String, Task> {
        ProofMapIndex::new(TASK_TABLE, self.view.clone())
    }

    /// Returns user for the given public key.
    pub fn task(&self, key: &PublicKey, task_name: &String) -> Option<Task> {
        self.taskes().get(&self.convert_to_task_key(key, task_name))
    }

    /// Create new user and append first record to its history.
    pub fn add_task(
        &mut self,
        student_key: &PublicKey,
        task_name: &String,
    ) {
        let created_task = {
            Task::new(
                student_key,
                task_name.clone()
            )
        };
        self.taskes().put(&self.convert_to_task_key(student_key, task_name), created_task);
    }

    ///
    fn convert_to_cert_key(&self, key: &PublicKey, course_name: &String) -> String {
        key.to_string() + &course_name.clone()
    }

    /// Returns `ProofMapIndex` with users.
    pub fn certes(&self) -> ProofMapIndex<T, String, Cert> {
        ProofMapIndex::new(CERT_TABLE, self.view.clone())
    }

    /// Returns user for the given public key.
    pub fn cert(&self, key: &PublicKey, course_name: &String) -> Option<Cert> {
        self.certes().get(&self.convert_to_cert_key(key, course_name))
    }

    /// Create new user and append first record to its history.
    pub fn add_cert(
        &mut self,
        student_key: &PublicKey,
        course_name: &String,
    ) {
        let created_cert = {
            Cert::new(
                student_key,
                course_name.clone()
            )
        };
        self.certes().put(&self.convert_to_cert_key(student_key, course_name), created_cert);
    }
}

//! Iphone queue database schema.
use crate::user::User;
use exonum::crypto::{Hash, PublicKey};
use exonum_merkledb::{IndexAccess, ObjectHash, ProofListIndex, ProofMapIndex};
use std::cmp::Ordering;

/// user table name
pub const USER_TABLE: &str = "educator.user";
/// Pipe type history table name
pub const USER_HISTORY_TABLE: &str = "educator.user.history";

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

    /// Returns history of the user with the given public key.
    // pub fn user_history(&self, public_key: &PublicKey) -> ProofListIndex<T, Hash> {
    //     ProofListIndex::new_in_family(USER_HISTORY_TABLE, public_key, self.view.clone())
    // }

    /// Returns user for the given public key.
    pub fn user(&self, pub_key: &PublicKey) -> Option<User> {
        self.users().get(pub_key)
    }

    /// Returns the state hash of service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.users().object_hash()]
    }

    // fn order_decs(&self, d1: &User, d2: &User) -> Ordering
    // {
    //     let sort_by_timestamp = d2.timestamp.cmp(&d1.timestamp);
    //     // if sort_by_timestamp != Ordering::Equal
    //     // {
    //     //     return sort_by_timestamp;
    //     // }
    //     sort_by_timestamp
    // }

    /// Returns first user.
    // pub fn first_user(&self) -> Option<User> {
    //     let users = self.users();
    //     let p = users.iter()
    //         .map(|x| x.1)
    //         .filter(|x| !x.have_bought && !x.removed)
    //         .max_by(|x, y| self.order_decs(x, y))
    //         .unwrap();
    //    Some(p)
    // }

    /// Create new user and append first record to its history.
    pub fn add_user(
        &mut self,
        key: &PublicKey,
    ) {
        let created_user = {
            User::new(
                key,
            )
        };
        self.users().put(key, created_user);
    }

    // /// User have bought a phone
    // pub fn user_have_bought(
    //     &mut self,
    //     user: User,
    //     transaction: &Hash
    // ) {
    //     let user = {
    //         let mut history = self.user_history(&user.key);
    //         history.push(*transaction);
    //         let history_hash = history.object_hash();
    //         user.buy(&history_hash)
    //     };
    //     self.users().put(&user.key, user.clone());
    // }

    // /// Remove a user.
    // pub fn remove_user(
    //     &mut self,
    //     user: User,
    //     transaction: &Hash
    // ) {
    //     let user = {
    //         let mut history = self.user_history(&user.key);
    //         history.push(*transaction);
    //         let history_hash = history.object_hash();
    //         user.remove(&history_hash)
    //     };
    //     self.users().put(&user.key, user.clone());
    // }
}

use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState, Error},
    blockchain::{self, BlockProof, TransactionMessage, },
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
    /// 
    pub pub_key: PublicKey,

    /// 
    pub course_name: String,
}

/// Describes the query parameters for the `get_cert` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmptyQuery {

}

/// Describes the query parameters for the `get_cert` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassQuery {
    /// 
    pub name: Option<String>,

}

/// Describes the query parameters for the `get_cert` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskQuery {
    /// 
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ClassObj {
    pub student_key: String,

    pub course_name: String,
}


/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    ///
    pub fn user_exist(state: &ServiceApiState, query: UserQuery) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);
        let user = schema.user(&query.pub_key);

        let answer = match user {
            None => false,
            Some(_i) => true,
        };

        Ok(answer)
    }

    ///
    pub fn get_cert(state: &ServiceApiState, query: CertQuery) -> api::Result<bool> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let &pub_key = &query.pub_key;
        let course_name = &query.course_name;

        // TODO LOGIC

        Ok(true)
    }

    ///
    pub fn get_courses(state: &ServiceApiState, query: EmptyQuery,) -> api::Result<Vec<&'static str>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        Ok(vec!["course_1", "course_2"])
    }

    ///
    pub fn get_classes(state: &ServiceApiState, query: ClassQuery) -> api::Result<Vec<&'static str>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let name = &query.name;

        let mut course_1 = vec!["class_1", "class_2"];
        let mut course_2 = vec!["class_1", "class_2", "class_3"];

        if name.is_none() {
            course_1.extend_from_slice(&course_2);
            Ok(course_1)
        } else {
            let res = name.as_ref().unwrap();

            if res == &String::from("course_1") {
                return Ok(course_1)
            }

            if res == &String::from("course_2") {
                return Ok(course_2)
            }
            Err(Error::NotFound(String::from("Course Name")))?
        }
    }

    ///
    pub fn get_tasks(state: &ServiceApiState, query: TaskQuery) -> api::Result<Vec<&'static str>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let name = &query.name;

        let mut course_1 = vec!["task_1_1", "task_1_2"];
        let mut course_2 = vec!["task_2_1", "task_2_2"];

        if name.is_none() {
            course_1.extend_from_slice(&course_2);
            Ok(course_1)
        } else {
            let res = name.as_ref().unwrap();

            if res == &String::from("course_1") {
                return Ok(course_1)
            }

            if res == &String::from("course_2") {
                return Ok(course_2)
            }
            Err(Error::NotFound(String::from("Course Name")))?
        }
    }

    ///
    pub fn get_classes_transactions(state: &ServiceApiState, query: EmptyQuery) -> api::Result<Vec<&'static str>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let name = &query.name;

        let mut course_1 = vec!["class_1", "class_2"];
        let mut course_2 = vec!["class_1", "class_2", "class_3"];

        if name.is_none() {
            course_1.extend_from_slice(&course_2);
            Ok(course_1)
        } else {
            let res = name.as_ref().unwrap();

            if res == &String::from("course_1") {
                return Ok(course_1)
            }

            if res == &String::from("course_2") {
                return Ok(course_2)
            }
            Err(Error::NotFound(String::from("Course Name")))?
        }
    }

    ///
    pub fn get_tasks_transactions(state: &ServiceApiState, query: EmptyQuery) -> api::Result<Vec<ClassObj>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        schema.tasks()()
    }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            // v1/educator/user_exist?pub_key={id}
            .endpoint("v1/educator/user_exist", Self::user_exist)
            // v1/educator/get_cert?pub_key={id}&course_name={name}
            .endpoint("v1/educator/get_courses", Self::get_courses)
            // v1/educator/get_courses?name={name}
            .endpoint("v1/educator/get_certs", Self::get_cert)
            // v1/educator/get_classes?name={name}
            .endpoint("v1/educator/get_classes", Self::get_classes)
            // v1/educator/get_tasks?name={name}
            .endpoint("v1/educator/get_tasks", Self::get_tasks)
            // v1/educator/get_classes?name={name}
            .endpoint("v1/educator/get_classes_transactions", Self::get_classes_transactions)
            // v1/educator/get_tasks?name={name}
            .endpoint("v1/educator/get_tasks_transactions", Self::get_tasks_transactions)
            ;
    }
}

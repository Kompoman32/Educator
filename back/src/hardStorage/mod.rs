#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

use exonum::crypto::PublicKey;

#[doc(hidden)]
pub fn can_add_class(&key: &PublicKey) -> bool {

    let add_class_allowed_users = vec![
        ""
    ];

    match add_class_allowed_users.iter().position(|&x| x == &key.to_string()) {
        None => false,
        _ => true,
    }
}

#[doc(hidden)]
pub fn can_add_task(&key: &PublicKey) -> bool {

    let add_task_allowed_users = vec![
        ""
    ];

    match add_task_allowed_users.iter().position(|&x| x == &key.to_string()) {
        None => false,
        _ => true,
    }
}

#[doc(hidden)]
pub fn can_add_user(&key: &PublicKey) -> bool {

    true

    // Uncomment when need to add access rights

    // let add_user_allowed_users = vec![
    //     ""
    // ];


    // match add_user_allowed_users.iter().position(|&x| x == &key.to_string()) {
    //     None => false,
    //     _ => true,
    // }
}

#[doc(hidden)]
pub fn can_add_cert(&key: &PublicKey) -> bool {

    true

    // Uncomment when need to add access rights

    // let add_cert_allowed_users = vec![
        
    // ];

    // if add_cert_allowed_users.len() == 0 {
    //     return true;
    // }

    // match add_cert_allowed_users.iter().position(|&x| x == &key.to_string()) {
    //     None => false,
    //     _ => true,
    // }
}
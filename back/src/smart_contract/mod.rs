#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

use exonum::crypto::PublicKey;

#[doc(hidden)]
pub fn can_get_cert(&student_key: &PublicKey, course_name: &String) -> bool {

    if course_name == &String::from("course_1") {
        return can_get_cert_in_course_1(&student_key);
    }

    if course_name == &String::from("course_2") {
        return can_get_cert_in_course_2(&student_key);
    }

    false
}

#[doc(hidden)]
fn can_get_cert_in_course_1(&student_key: &PublicKey) -> bool {
    false
}

#[doc(hidden)]
fn can_get_cert_in_course_2(&student_key: &PublicKey) -> bool {
    false
}

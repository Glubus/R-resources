#![cfg(r_resources_has_tests)]

use r_resources::include_resources;

include_resources!();

#[test]
fn test_r_tests_namespace_available() {
    // Values defined under res/tests/ should appear under r_tests::
    assert_eq!(r_tests::TEST_ONLY_MESSAGE, "This exists only in tests");
    assert_eq!(r_tests::TEST_LIMIT, 2_147_483_647_i32);
    assert!((r_tests::TEST_DECIMAL - 123.456).abs() < f64::EPSILON);
}


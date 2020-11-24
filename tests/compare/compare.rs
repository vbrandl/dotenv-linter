use crate::common::{check_output, TestDir};

#[test]
fn files_with_same_environment_variables() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc\nBAR=def");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = check_output(&[]);

    test_dir.test_command_success_with_args(
        &["compare", testfile_one.as_str(), testfile_two.as_str()],
        expected_output,
    );
}

#[test]
fn files_with_different_environment_variables() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = "file: \".env1\" is missing keys: [\"BAR\"]\n".to_owned();

    test_dir.test_command_fail_with_args(
        &["compare", testfile_one.as_str(), testfile_two.as_str()],
        expected_output,
    )
}

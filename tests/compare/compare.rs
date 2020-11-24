use crate::common::TestDir;

#[test]
fn files_with_same_environment_variables() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc\nBAR=def");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = format!("Collecting keys of file .env1\nCollecting keys of file .env2\n");

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
    let expected_output = format!("Collecting keys of file .env1\nCollecting keys of file .env2\n\".env1\" is missing keys: \"BAR\"\n");

    test_dir.test_command_fail_with_args(
        &["compare", testfile_one.as_str(), testfile_two.as_str()],
        expected_output,
    )
}

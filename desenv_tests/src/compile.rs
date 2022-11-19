use trybuild::TestCases;

#[test]
fn desenv_macro_usage_compilation_failures() {
    let test_cases: TestCases = TestCases::new();
    test_cases.compile_fail("test-assets/desenv/failure/*.rs");
}

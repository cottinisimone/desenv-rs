use trybuild::TestCases;

#[test]
fn desenv_macro_usage_compilation_failures() {
    let test_cases: TestCases = TestCases::new();
    test_cases.compile_fail("test-assets/desenv/failure/*.rs");
}

#[test]
fn struct_desenv_macro_attribute_compilation_success() {
    let test_cases: TestCases = TestCases::new();
    test_cases.pass("test-assets/struct/success/*.rs");
}

#[test]
fn struct_desenv_macro_attribute_compilation_failures() {
    let test_cases: TestCases = TestCases::new();
    test_cases.compile_fail("test-assets/struct/failure/*.rs");
}

#[test]
fn field_desenv_macro_attribute_compilation_success() {
    let test_cases: TestCases = TestCases::new();
    test_cases.pass("test-assets/field/success/*.rs");
}

#[test]
fn field_desenv_macro_attribute_compilation_failures() {
    let test_cases: TestCases = TestCases::new();
    test_cases.compile_fail("test-assets/field/failure/*.rs");
}

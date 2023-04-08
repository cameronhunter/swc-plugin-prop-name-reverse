use std::path::PathBuf;

use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::as_folder,
};
use swc_plugin_playground::TransformVisitor;
use testing::fixture;

#[fixture("tests/fixture/**/input.ts")]
fn fixture(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsConfig {
            tsx: input.to_string_lossy().ends_with(".tsx"),
            ..Default::default()
        }),
        &|_t| as_folder(TransformVisitor),
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

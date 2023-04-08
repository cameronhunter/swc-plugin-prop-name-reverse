use std::path::PathBuf;

use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            base::resolver,
            testing::{test_fixture, FixtureTestConfig},
        },
        visit::as_folder,
    },
};
use swc_plugin_playground::ObjectPropertyReverser;
use testing::fixture;

#[fixture("tests/fixture/**/input.ts")]
fn fixture(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsConfig {
            tsx: input.to_string_lossy().ends_with(".tsx"),
            ..Default::default()
        }),
        &|_t| {
            chain!(
                // See https://swc.rs/docs/plugin/ecmascript/cheatsheet#apply-resolver-while-testing
                resolver(Mark::new(), Mark::new(), false),
                as_folder(ObjectPropertyReverser)
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

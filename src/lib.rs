mod reverse;

use reverse::Reverse;
use swc_core::ecma::{ast::KeyValueProp, visit::VisitMut};

pub struct ObjectPropertyReverser;

impl VisitMut for ObjectPropertyReverser {
    fn visit_mut_key_value_prop(&mut self, key_value_prop: &mut KeyValueProp) {
        match key_value_prop.key.try_reverse() {
            Ok(prop_name) => {
                key_value_prop.key = prop_name;
            }
            Err(_) => {
                // Log error?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::ObjectPropertyReverser;
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
}

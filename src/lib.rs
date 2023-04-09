use std::{convert::Infallible, num::ParseFloatError};
use swc_core::ecma::{
    ast::{Ident, KeyValueProp, PropName},
    atoms::JsWord,
    visit::VisitMut,
};
use thiserror::Error;

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

trait Reverse {
    type Error;

    /// Try to reverse the value
    fn try_reverse(&self) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[derive(PartialEq, Debug, Error)]
enum PropNameReverseError {
    #[error("This should never have happened!")]
    Infallible {
        #[from]
        cause: Infallible,
    },

    #[error("Failed to reverse numeric property name")]
    NumberReverseError {
        #[from]
        cause: ParseFloatError,
    },

    #[error("Cannot reverse property type: %s")]
    Unsupported(&'static str),
}

impl Reverse for PropName {
    type Error = PropNameReverseError;

    fn try_reverse(&self) -> Result<Self, Self::Error> {
        match self {
            PropName::Ident(ident) => {
                let reversed = ident.sym.try_reverse()?;
                Ok(PropName::Ident(Ident::new(reversed.into(), ident.span)))
            }

            PropName::Num(number) => {
                let reversed = number.value.try_reverse()?;
                Ok(PropName::Num(reversed.into()))
            }

            PropName::Str(name) => {
                let reversed = name.value.try_reverse()?;
                Ok(PropName::Str(reversed.into()))
            }

            PropName::BigInt(_) => Err(Self::Error::Unsupported("BigInt")),
            PropName::Computed(_) => Err(Self::Error::Unsupported("Computed")),
        }
    }
}

impl Reverse for JsWord {
    type Error = Infallible;

    fn try_reverse(&self) -> Result<Self, Self::Error> {
        self.chars().rev().collect::<String>().try_into()
    }
}

impl Reverse for f64 {
    type Error = ParseFloatError;

    fn try_reverse(&self) -> Result<Self, Self::Error> {
        self.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<f64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_core::{common::DUMMY_SP, ecma::atoms::JsWord};

    #[test]
    fn test_reverse_prop_name_ident() {
        let input = PropName::Ident(Ident::new(JsWord::from("hello").into(), DUMMY_SP));
        let expected = PropName::Ident(Ident::new(JsWord::from("olleh").into(), DUMMY_SP));

        assert_eq!(input.try_reverse(), Ok(expected));
    }

    #[test]
    fn test_reverse_prop_name_num() {
        let input = PropName::Num(123456789.into());
        let expected = PropName::Num(987654321.into());

        assert_eq!(input.try_reverse(), Ok(expected));
    }

    #[test]
    fn test_reverse_prop_name_str() {
        let input = PropName::Str("hello".into());
        let expected = PropName::Str("olleh".into());

        assert_eq!(input.try_reverse(), Ok(expected));
    }

    #[test]
    fn test_reverse_f64() {
        assert_eq!(123456789f64.try_reverse(), Ok(987654321f64));
    }

    #[test]
    fn test_reverse_jsword() {
        let input: JsWord = "hello".into();
        let expected: JsWord = "olleh".into();

        assert_eq!(input.try_reverse(), Ok(expected));
    }
}

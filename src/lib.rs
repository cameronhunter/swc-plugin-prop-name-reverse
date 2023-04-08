use swc_core::ecma::{
    ast::{Ident, KeyValueProp, PropName},
    atoms::JsWord,
    visit::VisitMut,
};

pub struct ObjectPropertyReverser;

impl VisitMut for ObjectPropertyReverser {
    fn visit_mut_key_value_prop(&mut self, key_value_prop: &mut KeyValueProp) {
        if let Some(prop_name) = key_value_prop.key.reverse() {
            key_value_prop.key = prop_name;
        }
    }
}

trait Reverse {
    /// Reverse the value
    fn reverse(&self) -> Option<Self>
    where
        Self: Sized;
}

impl Reverse for PropName {
    fn reverse(&self) -> Option<Self> {
        match self {
            PropName::Ident(ident) => ident
                .sym
                .reverse()
                .map(|reversed| PropName::Ident(Ident::new(reversed.into(), ident.span))),

            PropName::Num(number) => number.value.reverse().map(|num| PropName::Num(num.into())),

            PropName::Str(name) => name
                .value
                .reverse()
                .map(|reversed| PropName::Str(reversed.into())),

            _ => None,
        }
    }
}

impl Reverse for JsWord {
    fn reverse(&self) -> Option<Self> {
        Some(self.chars().rev().collect::<String>().into())
    }
}

impl Reverse for f64 {
    fn reverse(&self) -> Option<Self> {
        self.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<f64>()
            .ok()
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

        assert_eq!(input.reverse(), Some(expected));
    }

    #[test]
    fn test_reverse_prop_name_num() {
        let input = PropName::Num(123456789.into());
        let expected = PropName::Num(987654321.into());

        assert_eq!(input.reverse(), Some(expected));
    }

    #[test]
    fn test_reverse_prop_name_str() {
        let input = PropName::Str("hello".into());
        let expected = PropName::Str("olleh".into());

        assert_eq!(input.reverse(), Some(expected));
    }

    #[test]
    fn test_reverse_f64() {
        assert_eq!(123456789f64.reverse(), Some(987654321f64));
    }

    #[test]
    fn test_reverse_jsword() {
        let input: JsWord = "hello".into();
        let expected: JsWord = "olleh".into();

        assert_eq!(input.reverse(), Some(expected));
    }
}

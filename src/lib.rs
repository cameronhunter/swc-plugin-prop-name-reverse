use swc_core::{
    common::Spanned,
    ecma::{
        ast::{op, BinExpr, Ident},
        visit::{VisitMut, VisitMutWith},
    },
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_bin_expr(&mut self, e: &mut BinExpr) {
        e.visit_mut_children_with(self);

        if e.op == op!("===") {
            e.left = Box::new(Ident::new("kdy1".into(), e.left.span()).into());
        }
    }
}

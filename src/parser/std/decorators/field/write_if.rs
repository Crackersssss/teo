use crate::core::field::builder::FieldBuilder;
use crate::core::field::Field;
use crate::core::field::write_rule::WriteRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn write_if_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    // field.write_rule = WriteRule::WriteIf(args.get(0).unwrap().resolved.unwrap().as_value().unwrap().as_pipeline().unwrap().clone());
}

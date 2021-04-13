use crate::ir;

pub fn evaluate(exp: ir::Expression) -> Option<u32> {
    match exp {
        ir::Expression::Constant(val) => match val {
            ir::Value::U32(tmp) => Some(tmp),
            ir::Value::I32(tmp) => Some(tmp as u32),
            ir::Value::Short(tmp) => Some(tmp as u32),
            ir::Value::UShort(tmp) => Some(tmp as u32),
        },
        _ => None,
    }
}

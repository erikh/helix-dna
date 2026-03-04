use crate::concept::{ResultOp, ResourceOp, Value};
use crate::error::NodeError;

/// Every concept node implements this to participate in execution.
pub trait Evaluate {
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, NodeError>;
    fn is_pure(&self) -> bool;
}

/// Pure computation node — always succeeds given valid inputs.
impl Evaluate for ResultOp {
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, NodeError> {
        match self {
            ResultOp::Const(v) => Ok(vec![Value::Int(*v)]),

            ResultOp::Add => {
                check_arity(inputs, 2)?;
                let a = require_int(&inputs[0])?;
                let b = require_int(&inputs[1])?;
                Ok(vec![Value::Int(a + b)])
            }

            ResultOp::Sub => {
                check_arity(inputs, 2)?;
                let a = require_int(&inputs[0])?;
                let b = require_int(&inputs[1])?;
                Ok(vec![Value::Int(a - b)])
            }

            ResultOp::Mul => {
                check_arity(inputs, 2)?;
                let a = require_int(&inputs[0])?;
                let b = require_int(&inputs[1])?;
                Ok(vec![Value::Int(a * b)])
            }

            ResultOp::Div => {
                check_arity(inputs, 2)?;
                let a = require_int(&inputs[0])?;
                let b = require_int(&inputs[1])?;
                if b == 0 {
                    return Err(NodeError::DivisionByZero);
                }
                Ok(vec![Value::Int(a / b)])
            }

            ResultOp::Eq => {
                check_arity(inputs, 2)?;
                let a = require_int(&inputs[0])?;
                let b = require_int(&inputs[1])?;
                Ok(vec![Value::Bool(a == b)])
            }

            ResultOp::Identity => {
                check_arity(inputs, 1)?;
                Ok(vec![inputs[0].clone()])
            }
        }
    }

    fn is_pure(&self) -> bool {
        true
    }
}

/// Resource node — can fail, maps to WASI syscalls.
impl Evaluate for ResourceOp {
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, NodeError> {
        match self {
            ResourceOp::FdWrite { fd } => {
                check_arity(inputs, 1)?;
                // In concept-level evaluation, we simulate success.
                // Actual WASI calls happen at the WASM level.
                let _ = fd;
                Ok(vec![Value::Unit])
            }

            ResourceOp::FdRead { fd } => {
                let _ = fd;
                // Simulate reading — returns empty bytes.
                Ok(vec![Value::Bytes(Vec::new())])
            }

            ResourceOp::PathOpen { path } => {
                // Simulate — in reality this would be a WASI call that can fail.
                let _ = path;
                Err(NodeError::ResourceUnavailable(
                    "path_open not available in concept evaluation".to_string(),
                ))
            }
        }
    }

    fn is_pure(&self) -> bool {
        false
    }
}

fn check_arity(inputs: &[Value], expected: usize) -> Result<(), NodeError> {
    if inputs.len() != expected {
        return Err(NodeError::ArityMismatch {
            expected,
            got: inputs.len(),
        });
    }
    Ok(())
}

fn require_int(value: &Value) -> Result<i64, NodeError> {
    value.as_int().ok_or_else(|| NodeError::TypeMismatch {
        expected: "Int".into(),
        got: format!("{value:?}"),
    })
}

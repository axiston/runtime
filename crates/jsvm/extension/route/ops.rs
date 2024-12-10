use deno_core::op2;

use crate::extension::route::Result;

#[op2(fast)]
pub fn op_register_service() -> Result<()> {
    Ok(())
}

#[op2(fast)]
pub fn op_register_trigger() -> Result<()> {
    Ok(())
}

#[op2(fast)]
pub fn op_register_action() -> Result<()> {
    Ok(())
}

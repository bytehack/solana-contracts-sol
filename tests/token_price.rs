use c8ntinuum::{token_price, CustomError};

#[test]
fn token_price_empty_pool_error() {
    let result = token_price(0, 1);
    assert!(matches!(result, Err(anchor_lang::prelude::ProgramError::Custom(code)) if code == CustomError::InvalidPoolState as u32));
}

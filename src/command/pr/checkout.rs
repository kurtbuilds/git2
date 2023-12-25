use clap::Parser;

use crate::prelude::*;

#[derive(Parser, Debug)]
pub struct Checkout {
    number: String,
}

impl Checkout {
    pub fn run(self) -> Result {
        Ok(())
    }
}
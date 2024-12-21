#![allow(dead_code)]

pub mod commands;

use crate::{cashcode::CashCode, config::Options};
use std::error::Error;

pub struct App {
    pub cashcode: CashCode,
}

impl App {
    pub fn build(options: Options) -> Result<Self, Box<dyn Error>> {
        let serial = CashCode::build(&options.path, options.baud_rate)?;

        Ok(Self { cashcode: serial })
    }
}

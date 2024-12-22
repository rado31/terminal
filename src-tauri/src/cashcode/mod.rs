#![allow(dead_code)]

mod commands;

use serialport::{DataBits, Error, FlowControl, Parity, SerialPort, StopBits};
use std::{sync::Mutex, thread, time::Duration};

pub struct CashCode {
    port: Mutex<Box<dyn SerialPort>>,
    response: Vec<u8>,
}

impl CashCode {
    pub fn build(path: &str, baud_rate: u32) -> Result<Self, Error> {
        let port = serialport::new(path, baud_rate)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_millis(100))
            .open()?;

        Ok(Self { port: Mutex::new(port), response: vec![0; 6] })
    }

    pub fn disable(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::DISABLE) {
            log::error!("DISABLE write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("DISABLE read: {error}");
        };

        self.response = vec![0; 6];
    }

    pub fn enable(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::ENABLE) {
            log::error!("ENABLE write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("ENABLE read: {error}");
        };

        self.response = vec![0; 6];
    }

    pub fn reset(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::RESET) {
            log::error!("RESET write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("RESET read: {error}");
        };

        self.response = vec![0; 6];
    }

    pub fn set_security(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::SECURITY) {
            log::error!("SET SECURITY write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("SET SECURITY read: {error}");
        };

        thread::sleep(Duration::from_secs(2));
        self.response = vec![0; 6];
    }

    pub fn return_bill(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::RETURN) {
            log::error!("RETURN write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("RETURN read: {error}");
        };

        self.response = vec![0; 6];
    }

    pub fn stack(&mut self) {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::STACK) {
            log::error!("STACK write: {error}");
        };

        if let Err(error) = port.read(self.response.as_mut_slice()) {
            log::error!("STACK read: {error}");
        };

        self.response = vec![0; 6];
    }

    pub fn poll(&mut self) -> [u8; 2] {
        let mut port = self.port.lock().unwrap();

        if let Err(error) = port.write(commands::POLL) {
            log::error!("POLL WRITE: {error}");
        };

        thread::sleep(Duration::from_millis(10));

        let mut response: Vec<u8> = vec![];

        loop {
            let mut byte: Vec<u8> = vec![0; 1];

            match port.read(byte.as_mut_slice()) {
                Ok(b) if b == 1 => response.push(byte[0]),
                Ok(_) => break,
                Err(_) => break,
            };
        }

        if let Err(error) = port.write(commands::ACK) {
            log::error!("POLL ACK: {error}");
        };

        thread::sleep(Duration::from_secs(1));

        let mut result: [u8; 2] = [0; 2];
        result.copy_from_slice(&response[3..5]);

        result
    }

    pub fn bill_types(bill: u8) -> i32 {
        match bill {
            0 => 1,
            2 => 5,
            3 => 10,
            4 => 20,
            5 => 50,
            6 => 100,
            _ => 0,
        }
    }
}

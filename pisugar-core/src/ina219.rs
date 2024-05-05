use rppal::i2c::I2c;

/// INA219 I2C addr
pub const I2C_ADDR_INA219: u16 = 0x41;

use crate::{CurrentSensor, PiSugarConfig, Result};

enum Register {
    // Configuration = 0x00,
    ShuntVoltage = 0x01,
    BusVoltage = 0x02,
    Power = 0x03,
    Current = 0x04,
    Calibration = 0x05,
}

pub struct INA219 {
    i2c: I2c,
}

impl INA219 {
    /// Init
    pub fn new(cfg: PiSugarConfig) -> Result<Self> {
        let mut i2c = I2c::with_bus(cfg.i2c_bus)?;
        i2c.set_slave_address(I2C_ADDR_INA219)?;
        Ok(Self { i2c })
    }

    fn read(&self, register: Register) -> Result<u16> {
        let v = self.i2c.smbus_read_word_swapped(register as u8)?;
        Ok(v)
    }
}

impl CurrentSensor for INA219 {
    /// Init
    fn init(&mut self, _config: &PiSugarConfig) -> Result<()> {
        Ok(())
    }

    fn calibrate(&self, value: u16) -> Result<()> {
        self.i2c
            .smbus_write_word_swapped(Register::Calibration as u8, value as u16)?;
        Ok(())
    }

    fn shunt_voltage(&self) -> Result<i16> {
        let value = self.read(Register::ShuntVoltage)?;
        Ok(value as i16)
    }

    fn voltage(&self) -> Result<u16> {
        let value = self.read(Register::BusVoltage)?;
        Ok((value >> 3) * 4)
    }

    fn power(&self) -> Result<i16> {
        let value = self.read(Register::Power)?;
        Ok(value as i16)
    }

    fn current(&self) -> Result<i16> {
        let value = self.read(Register::Current)?;
        Ok(value as i16)
    }
}

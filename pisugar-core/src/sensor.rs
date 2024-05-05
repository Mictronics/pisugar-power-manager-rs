use crate::{PiSugarConfig, Result};

/// INA219 current sensor trait
pub trait CurrentSensor {
    fn init(&mut self, config: &PiSugarConfig) -> Result<()>;
    fn shunt_voltage(&self) -> Result<i16>;
    fn voltage(&self) -> Result<u16>;
    fn power(&self) -> Result<i16>;
    fn current(&self) -> Result<i16>;
    fn calibrate(&self, value: u16) -> Result<()>;
}

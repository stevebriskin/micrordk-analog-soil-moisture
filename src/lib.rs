use micro_rdk::common::config::ConfigType;
use micro_rdk::common::registry::{
    get_board_from_dependencies, ComponentRegistry, Dependency, RegistryError,
};
use micro_rdk::common::status::{Status, StatusError};
use micro_rdk::DoCommand;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use micro_rdk::common::sensor::{
    GenericReadingsResult, Readings, Sensor, SensorError, SensorResult, SensorT, SensorType,
    TypedReadingsResult,
};

use std::thread;
use std::time;

pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    registry.register_sensor("soil_moisture", &SoilMoistureSensor::from_config)?;
    Ok(())
}

#[derive(DoCommand)]
pub struct SoilMoistureSensor {
    reader: AnalogReaderType<u16>,
    num_readings: i32,
}

impl SoilMoistureSensor {
    pub fn from_config(cfg: ConfigType, deps: Vec<Dependency>) -> Result<SensorType, SensorError> {
        let board = get_board_from_dependencies(deps);
        if board.is_none() {
            return Err(SensorError::ConfigError("sensor missing board attribute"));
        }
        let board_unwrapped = board.unwrap();

        let num_readings = cfg.get_attribute::<i32>("num_readings").unwrap_or(5);

        if num_readings < 1 {
            return Err(SensorError::ConfigError(
                "num_readings must be an integer greater than 1",
            ));
        }

        if let Ok(analog_reader_name) = cfg.get_attribute::<String>("analog_reader") {
            if let Ok(reader) = board_unwrapped.get_analog_reader_by_name(analog_reader_name) {
                Ok(Arc::new(Mutex::new(Self {
                    reader,
                    num_readings,
                })))
            } else {
                Err(SensorError::ConfigError("failed to get analog reader"))
            }
        } else {
            Err(SensorError::ConfigError(
                "failed to get 'analog_reader' value from config",
            ))
        }
    }
}

impl Status for SoilMoistureSensor {
    fn get_status(&self) -> Result<Option<micro_rdk::google::protobuf::Struct>, StatusError> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}

impl Sensor for SoilMoistureSensor {}

impl Readings for SoilMoistureSensor {
    fn get_generic_readings(&mut self) -> Result<GenericReadingsResult, SensorError> {
        Ok(self
            .get_readings()?
            .into_iter()
            .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
            .collect())
    }
}

impl SensorT<f64> for SoilMoistureSensor {
    fn get_readings(&self) -> Result<TypedReadingsResult<f64>, SensorError> {
        let mut readings = Vec::new();
        for i in 0..self.num_readings {
            let reading = self
                .reader
                .lock()
                .map_err(|_| SensorError::SensorGenericError("failed to get sensor lock"))?
                .read()?;
            readings.push(reading as i16);

            if i < self.num_readings - 1 {
                // Small delay between readings
                thread::sleep(time::Duration::from_millis(1));
            }
        }

        // Calculate median
        readings.sort();
        let mid = readings.len() / 2;
        let median_reading = readings[mid];

        let mut results = HashMap::new();
        results.insert("moisture_raw".to_string(), median_reading as f64);
        results.insert("num_readings".to_string(), readings.len() as f64); // Number of readings taken for debugging

        Ok(results)
    }
}

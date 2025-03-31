# MicroRDK Analog Soil Moisture Sensor

This module adds support for the capacitative analog moisture sensor.

## Build
See https://docs.viam.com/operate/get-started/other-hardware/micro-module/

## Configure

The soil sensor requires an analog configuration on the board. 

Example:
```
        "analogs": [
          {
            "name": "soil",
            "pin": 35
          }
        ]
```

Sensor configuration:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| analog_reader   | <string> | Yes | The name of the analog reader on the board configuration. |
| num_readings | <int> | No | Number of raw readings to take per returned median value |
| wet_value | <int> | No | Calibration value that maps to fully wet (100) |
| dry_value | <int> | No | Calibration value that maps to fully dry (0) |

Example:
```
{
    "analog_reader": "soil",
    "num_readings": 15,
}
```

## Returned Values
| Key | Type | Description |
|-----|------|-------------|
| moisture_raw | float | Raw analog reading representing soil moisture. |
| num_readings | float | Number of readings taken for this measurement. |
| moisture_mapped | float | A linearlyed mapped value [0,100] based on optional wet_value and dry_value in the config|

# Calibration

Standard calibration of capacitative moisture sensors should be followed if mapped output values are desired.
Obtain the "moisture_raw" reading of the sensor completely dry.
Obtain the "moisture_raw" reading of the sensor completely wet (e.g. submerged in water).
Enter both values as "dry_value" and "wet_value" in the configuration. Sensor values will then be mapped to a linear scale between 0 and 100 in "moisture_mapped", where raw values <= "dry_value" will be 0 and values >= "wet_value" will be 100.
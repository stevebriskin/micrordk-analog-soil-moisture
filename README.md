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

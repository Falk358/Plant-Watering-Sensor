# Features Planned

## Calibration
We dont know the circumstances the sensor will be deployed to, therefore Calibration will be necessary. We suggest measuring the value when the earth is completely dry (maximal value) and completely wet (minimal).

+ [x] Measure max_value of sensor in earth (dry)
+ [x] Measure min_value of sensor in earth (wet)
+ [ ] store values in persistent memory (EEPROM); load them for the main application on startup.
      + does our controller have EEPROM?
      + how much memory is available? can it store two u16?
      

## Think about UX-Library

+ [ ] Implement turn on led
+ [ ] Implement read button
+ [ ] Change colors of led
+ [ ] Let the led blink

## State Machine
A state machine should help structure the program and improve debugability. 

used crate: [sfsm](https://docs.rs/sfsm/0.4.3/sfsm/)




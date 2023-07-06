# pragma once

# include <Arduino.h>

/// @brief Writes the given struct to the serial for telemetry purposes
/// @tparam T The struct type that will be written to the serial
/// @param serial serial interface to write to
/// @param s struct that will be written to the serial
template<typename T>
void write_struct(const T* s) {
    const uint8_t* array = (const uint8_t*)s;
    Serial.write(array, sizeof(T));
}
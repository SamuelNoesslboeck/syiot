# pragma once

# include <Arduino.h>

namespace telemetry {
    /// @brief The buffer used to store telemetry input
    static uint8_t BUF [16];
    
    /// @brief Writes the given struct to the serial for telemetry purposes
    /// @tparam T The struct type that will be written to the serial
    /// @param serial serial interface to write to
    /// @param s struct that will be written to the serial
    template<typename T>
    void write_struct(const T* s) {
        const uint8_t* array = (const uint8_t*)s;
        Serial.write(array, sizeof(T));
    }

    /// @brief Checks if an update is requested (Serial input)
    /// @tparam T The type of the telemetry data
    /// @param s The instance of the telemetry data
    template<typename T>
    void update(const T* s) {
        size_t bytes_len = Serial.readBytes(BUF, Serial.available());

        if (bytes_len > 0) {
            write_struct(&magicbox::state());
        }
    }
}
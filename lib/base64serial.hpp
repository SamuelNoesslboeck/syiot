# pragma once

# include <Arduino.h>

template<typename T>
void write_base64_serial(Serial* serial, T* val);
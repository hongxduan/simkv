//
// Created by HongXing on 2/10/25.
//

#ifndef SIMKV_BYTE_UTIL_H
#define SIMKV_BYTE_UTIL_H
#include <cstdint>

namespace util {
    //
    // convert int32 to bytes[4]
    //
    void int32_to_bytes(int32_t value, uint8_t bytes[4]);

    //
    // convert uint32 to bytes[4]
    //
    void uint32_to_bytes(uint32_t value, uint8_t bytes[4]);

    //
    // convert little endian bytes to uint32
    //
    int32_t bytes_to_int32(const uint8_t *bytes);

    //
    // convert little endian bytes to uint32
    //
    uint32_t bytes_to_uint32(const uint8_t *bytes);

    //
    // convert little endian bytes to uint6
    //
    uint16_t bytes_to_uint16(const uint8_t *bytes);
}


#endif //SIMKV_BYTE_UTIL_H

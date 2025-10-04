//
// Created by HongXing on 2/10/25.
//

#ifndef SIMKV_BYTE_UTIL_H
#define SIMKV_BYTE_UTIL_H
#include <cstdint>

namespace util {
    //
    // convert little endian bytes to uint32
    //
    uint32_t le_bytes_to_uint32(const uint8_t *bytes);

    //
    // convert little endian bytes to uint6
    //
    uint16_t le_bytes_to_uint16(const uint8_t *bytes);
}



#endif //SIMKV_BYTE_UTIL_H
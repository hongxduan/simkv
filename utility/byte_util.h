//
// Created by HongXing on 2/10/25.
//

#ifndef SIMKV_BYTE_UTIL_H
#define SIMKV_BYTE_UTIL_H
#include <cstdint>

namespace utility {
    //
    // convert little endian bytes to uint32
    //
    uint32_t le_bytes_to_uint32(uint8_t *bytes);
}



#endif //SIMKV_BYTE_UTIL_H
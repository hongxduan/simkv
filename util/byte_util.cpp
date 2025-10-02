//
// Created by HongXing on 2/10/25.
//

#include <bit>
#include "byte_util.h"

#include <iterator>

uint32_t util::le_bytes_to_uint32(uint8_t *bytes) {
    // convert bytes to int first
    uint32_t result = bytes[0] << 24 | bytes[1] << 16 | bytes[2] << 8 | bytes[3];
    //std::memcpy(&reuslt, bytes, sizeof(result));

    // if the System is Big Endian, then need swap bytes
    if (std::endian::big == std::endian::native) {
        return std::byteswap(result);
    }
    return result;
}

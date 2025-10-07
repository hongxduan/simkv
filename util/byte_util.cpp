//
// Created by HongXing on 2/10/25.
//

#include <bit>
#include "byte_util.h"

#include <iterator>

int32_t util::bytes_to_int32(const uint8_t *bytes) {
    // convert bytes to int first
    const uint32_t result = bytes[0] << 24 | bytes[1] << 16 | bytes[2] << 8 | bytes[3];
    //std::memcpy(&reuslt, bytes, sizeof(result));

    return result;
}

uint32_t util::bytes_to_uint32(const uint8_t *bytes) {
    // convert bytes to int first
    const uint32_t result = bytes[0] << 24 | bytes[1] << 16 | bytes[2] << 8 | bytes[3];
    //std::memcpy(&reuslt, bytes, sizeof(result));

    return result;
}


uint16_t util::bytes_to_uint16(const uint8_t *bytes) {
    // convert bytes to int first
    const uint16_t result = bytes[0] << 8 | bytes[1];
    //std::memcpy(&reuslt, bytes, sizeof(result));

    return result;
}

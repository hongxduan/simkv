//
// Created by HongXing on 7/10/25.
//

#ifndef SIMKV_CRC_UTIL_H
#define SIMKV_CRC_UTIL_H


#define CRC16_POLY 0x8005
#include <string>

namespace util {
    //
    // CRC16 hash
    //
    uint16_t crc16(const std::string& str);
}

#endif //SIMKV_CRC_UTIL_H
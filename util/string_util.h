//
// Created by HongXing on 29/9/25.
//

#ifndef SIMKV_STRING_UTILITY_H
#define SIMKV_STRING_UTILITY_H

#define CRC16_POLY 0x8005

namespace util {
    std::string trim_left(const std::string &str);

    std::string trim_right(const std::string &str);

    std::string trim(const std::string &str);

    //
    // CRC16 hash
    //
    uint16_t crc16(const std::string str, uint16_t size);
}

#endif //SIMKV_STRING_UTILITY_H

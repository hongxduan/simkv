//
// Created by HongXing on 7/10/25.
//

#include "crc_util.h"

uint16_t util::crc16(const std::string &str) {
    uint16_t crc = 0;
    auto *data = str.c_str();
    uint16_t size = strlen(data);
    //std::cout << "size: " << size << std::endl;
    while (size--) {
        crc ^= *data++ << 8;
        for (int i = 0; i < 8; ++i) {
            if (crc & 0x8000)
                crc = (crc << 1) ^ CRC16_POLY;
            else
                crc <<= 1;
        }
    }
    return crc;
}

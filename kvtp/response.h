//
// Created by HongXing on 1/10/25.
//

#ifndef SIMKV_RESPONSE_H
#define SIMKV_RESPONSE_H
#include <vector>

#include "../inc/type.h"


namespace kvtp {
    //
    // encode KVTP response
    //
    std::vector<uint8_t> encode_response();

    //
    // encode i32 to kvtp response
    //
    std::vector<BYTE> encode_i32_response(int val);

    //
    // encode string to kvtp response
    //
    std::vector<BYTE> encode_str_response();

    void append_ok(std::vector<BYTE> &response);
}

#endif //SIMKV_RESPONSE_H

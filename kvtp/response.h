//
// Created by HongXing on 1/10/25.
//

#ifndef SIMKV_RESPONSE_H
#define SIMKV_RESPONSE_H
#include <vector>

#include "../inc/type.h"


namespace kvtp {
    const std::vector<BYTE> resp_type_I = {'I'};
    const std::vector<BYTE> resp_type_S = {'S'};
    const std::vector<BYTE> resp_type_D = {'D'};


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
    std::vector<BYTE> encode_str_response(std::string val);

    //
    // build ok header, including protocol, data type, and separtor
    //
    void append_ok_header(std::vector<BYTE> &response, std::vector<BYTE> resp_type);

    //
    // encode error response
    //
    std::vector<BYTE> encode_err_response(std::string val);

    //
    // prepend 4 bytes reponse length
    //
    void prepend_len_bytes(std::vector<BYTE> &response);
}

#endif //SIMKV_RESPONSE_H

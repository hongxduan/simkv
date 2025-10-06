//
// Created by HongXing on 1/10/25.
//

#include "response.h"

#include "kvtp.h"

std::vector<uint8_t> kvtp::encode_response() {
    std::vector<uint8_t> response;


    return response;
}

std::vector<BYTE> kvtp::encode_i32_response(int val) {
    std::vector<BYTE> response;

    append_ok(response);


    return response;
}


void kvtp::append_ok(std::vector<BYTE> &response) {
    std::string protocol = PROTOCOL_V1;
    std::vector<BYTE> protocol_bytes ;
    protocol_bytes.assign(protocol.begin(), protocol.end());
    response.append_range(protocol);
}

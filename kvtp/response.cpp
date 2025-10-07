//
// Created by HongXing on 1/10/25.
//

#include "response.h"

#include <iterator>

#include "kvtp.h"
#include "../util/byte_util.h"

std::vector<uint8_t> kvtp::encode_response() {
    std::vector<uint8_t> response;


    return response;
}

std::vector<BYTE> kvtp::encode_i32_response(const int val) {
    std::vector<BYTE> response;

    // header
    append_ok_header(response, resp_type_I);

    // body
    BYTE bytes[4];
    util::int32_to_bytes(val, bytes);
    response.append_range(bytes);

    // len
    prepend_len_bytes(response);

    return response;
}

std::vector<BYTE> kvtp::encode_str_response(std::string val) {
    std::vector<BYTE> response;

    // header
    append_ok_header(response, resp_type_S);

    // body
    response.append_range(val);

    // len
    prepend_len_bytes(response);

    return response;
}


void kvtp::append_ok_header(std::vector<BYTE> &response, std::vector<BYTE> resp_type) {
    // protocol
    std::string protocol = PROTOCOL_V1;
    std::vector<BYTE> protocol_bytes ;
    protocol_bytes.assign(protocol.begin(), protocol.end());
    response.append_range(protocol);
    response.push_back(LINE_FEED);

    // data type
    response.append_range(resp_type);
    response.push_back(LINE_FEED);

    // separator
    response.push_back(LINE_FEED);
}

void kvtp::prepend_len_bytes(std::vector<BYTE> &response) {
    uint32_t len = response.size();
    BYTE len_bytes[4];
    util::uint32_to_bytes(len, len_bytes);
    response.append_range(len_bytes);
}


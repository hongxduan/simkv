//
// Created by HongXing on 30/9/25.
//

#include <iostream>
#include <sstream>
#include "request.h"

#include "response.h"
#include "../util/byte_util.h"

#define KEY_WIDTH 2 // bytes of key

kvtp::KvtpRequest kvtp::decode_request(std::vector<BYTE> raw_req) {
    // set initial value is important, or read dirty memroy
    kvtp::KvtpRequest kvtp_req={
         "",
         "",
        "",
        "",
        0,
        {},
        false
    };

    std::string raw_str = std::string(raw_req.begin(), raw_req.end());
    std::stringstream stream(raw_str);
    std::string tmp;
    size_t line_num = 0;
    size_t header_size = 0;

    //
    // decode header
    //
    while (std::getline(stream, tmp, LINE_FEED)) {
        // accumulate header size
        header_size += tmp.size();
        header_size += 1; // ending '\0'

        if (line_num == 0) {
            kvtp_req.protocol = tmp;
        } else {
            // if meet empty line, i.e. header and body seperator
            // then end the while loop
            if (tmp == "") {
                break;
            }

            std::stringstream ss_line(tmp);
            std::string part;
            std::string head_key, head_val;
            size_t part_index = 0;
            //
            // split comma seperated header key:val
            //
            while (std::getline(ss_line, part, COLON)) {
                if (part_index == 0) {
                    head_key = part;
                } else if (part_index == 1) {
                    head_val = part;
                }
                part_index++;
            }

            //
            // check all head_key
            //
            if (head_key == CMD_PREFIX) {
                kvtp_req.cmd = head_val;
            } else if (head_key == KEY_PREFIX) {
                kvtp_req.key = head_val;
            } else if (head_key == ARGS_PREFIX) {
                kvtp_req.args = head_val;
            } else if (head_key == TTL_PREFIX) {
                int32_t ttl = atol(head_val.c_str());
                // todo: what if ttl = 0?
                kvtp_req.ttl = ttl;
            }
        }
        line_num++;
    }

    //
    // decode body
    // body consits of 2 bytes of key lenght,
    // followed by key bytes, and then body bytes
    //
    std::vector<uint8_t> body_bytes;
    body_bytes.assign(raw_req.begin() + header_size, raw_req.end());

    // decode key_size, i.e. total bytes count of key
    BYTE key_size_bytes[KEY_WIDTH] = {body_bytes[0], body_bytes[1]};
    uint16_t key_size = util::bytes_to_uint16(key_size_bytes);

    // kvtp format error
    if (key_size>=body_bytes.size()) {
        kvtp_req.error = true;
        return kvtp_req;
    }

    // key string
    std::string key = std::string(body_bytes.begin() + KEY_WIDTH, body_bytes.begin() + KEY_WIDTH + key_size);
    kvtp_req.key = key;

    // value bytes
    std::vector<BYTE> val_bytes;
    val_bytes.assign(body_bytes.begin() + KEY_WIDTH + key_size, body_bytes.end());
    kvtp_req.val = val_bytes;

    /************************************************
     * for debug
     ************************************************/
    //std::string val = std::string(val_bytes.begin(), val_bytes.end());
    //std::cout << "key from body:" << key << std::endl;
    //std::cout << "val from body:" << val << std::endl;


    return kvtp_req;
}

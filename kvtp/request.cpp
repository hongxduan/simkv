//
// Created by HongXing on 30/9/25.
//

#include <iostream>
#include <sstream>
#include "request.h"

kvtp::KvtpRequest kvtp::decode_request(std::vector<BYTE> raw_req) {
    kvtp::KvtpRequest kvtp_req;

    std::string raw_str = std::string(raw_req.begin(), raw_req.end());
    std::stringstream stream(raw_str);
    std::string tmp;
    size_t line_num = 0;
    while (std::getline(stream, tmp, LINE_FEED)) {
        if (line_num==0) {
            kvtp_req.protocol = tmp;
        }else {
            std::stringstream ss_line(tmp);
            std::string part;
            std::string head_key, head_val;
            size_t part_index = 0;
            //
            // split comma seperated header key:val
            //
            while (std::getline(ss_line, part, COLON)) {
                if (part_index==0) {
                    head_key = part;
                }else if (part_index==1) {
                    head_val = part;
                }
                part_index++;
            }

            //
            // check all head_key
            //
            if (head_key == CMD_PREFIX) {
                kvtp_req.cmd = head_val;
            }else if (head_key == KEY_PREFIX) {
                kvtp_req.key = head_val;
            }else if (head_key == ARGS_PREFIX) {
                kvtp_req.args = head_val;
            }else if (head_key == TTL_PREFIX) {
                int32_t ttl = atol(head_val.c_str());
                // todo: what if ttl = 0?
                kvtp_req.ttl = ttl;
            }
        }
        line_num++;
    }

    std::cout << kvtp_req.cmd << " " << kvtp_req.key << std::endl;

    return kvtp_req;
}

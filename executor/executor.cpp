//
// Created by HongXing on 30/9/25.
//

#include <iostream>
#include "executor.h"

#include "../inc/type.h"
#include "../kvtp/request.h"

std::vector<BYTE> execute(std::vector<BYTE> raw_req) {
    std::vector<BYTE> result;

    std::cout << "message:\n" << raw_req.data() << std::endl;
    // decode raw req to kvtp req
    auto kvtp_req = kvtp::decode_request(raw_req);

    //

    return result;
}

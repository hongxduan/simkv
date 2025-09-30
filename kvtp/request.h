//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_REQUEST_H
#define SIMKV_REQUEST_H

#include <vector>
/*
 * Key-Value Transport Protocol Header
 *
 *
 *
 *
 */
namespace kvtp {
    /*
     * Kvtp request data type
     */
    struct KvtpRequest {
        /* the protocol */
        std::string protocol;

        /* the string command */
        std::string cmd;

        /* the key */
        std::string key;

        /* arguments */
        std::string args;

        /* time to live */
        uint32_t ttl;

        /* reqeust body */
        std::vector<uint8_t> body;
    };

    /*
     * parse request from client to Kvtp Request
     */
    KvtpRequest parseRequest(std::vector<uint8_t> request);
}


#endif //SIMKV_REQUEST_H

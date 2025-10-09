//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_REQUEST_H
#define SIMKV_REQUEST_H

#include <vector>

#include "../inc/type.h"


/****************** KVTP Request ******************************
 * KVTP/1              ...... Protocol (Must be the first line)
 * CMD: GET|SET        ...... Command
 * KEY: k1|users[1]    ...... Key
 * ARGS: EX|NX         ...... Arguments (Optional)
 * TTL: 0              ...... Time To Live (Optional)
 *                     ...... Empty line(Header Body separator)
 * Body                ...... 00000000 00000000keyval
***************************************************************/

/*
 * Of body, the first 2 bytes are length of key, continue with key and value bytes
 *
 */

#define ZERO '\0'
#define COLON ':'

#define LINE_FEED '\n'

#define PROTOCOL "KVTP/1"
#define KEY_PREFIX "KEY"
#define CMD_PREFIX "CMD"
#define ARGS_PREFIX "ARGS"
#define TTL_PREFIX "TTL"

//
// Key-Value Transport Protocol Header
//
namespace kvtp {
    // Kvtp request data type
    struct KvtpRequest {
        // the protocol
        std::string protocol;

        // the string command
        std::string cmd;

        // the key
        std::string key;

        // arguments
        std::string args;

        // time to live
        int64_t ttl;

        // reqeust body
        std::vector<uint8_t> val;

        bool error;

        std::string error_msg;
    };

    //
    // decode request from client to Kvtp Request
    //
    KvtpRequest decode_request(std::vector<BYTE> raw_req);
}


#endif //SIMKV_REQUEST_H

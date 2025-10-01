//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_REQUEST_H
#define SIMKV_REQUEST_H

#include <vector>


/****************** KVTP Request ****************************
KVTP/1              ...... Protocol (Must be the first line)
CMD: GET|SET        ...... Command
KEY: k1|users[1]    ...... Key
ARGS: EX|NX         ...... Arguments (Optional)
TTL: 0              ...... Time To Live (Optional)
                    ...... Empty line(Header Body separator)
Body                ...... Body
************************************************************/

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
        uint32_t ttl;

        // reqeust body
        std::vector<uint8_t> body;
    };

    //
    // decode request from client to Kvtp Request
    //
    KvtpRequest decodeRequest(std::vector<uint8_t> request);
}


#endif //SIMKV_REQUEST_H

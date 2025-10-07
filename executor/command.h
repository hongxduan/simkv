//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_COMMAND_H
#define SIMKV_COMMAND_H

namespace cmd {
    //#define GET "GET"
    //#define SET "SET"
    //#define DEL "DEL"
    //#define KEY "KEY"

    const std::string GET = "GET";
    const std::string SET = "SET";
    const std::string DEL = "DEL";
    const std::string KEY = "KEY";
}

const std::string ok_rslt = "OK";
const std::string err_rslt = "ERR";
const std::string key_not_found_rslt = "KEY_NOT_FOUND";
#endif //SIMKV_COMMAND_H

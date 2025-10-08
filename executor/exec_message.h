//
// Created by HongXing on 8/10/25.
//

#ifndef SIMKV_EXEC_MESSAGE_H
#define SIMKV_EXEC_MESSAGE_H

#include <string>

namespace execmsg {
    const std::string OK = "OK";
    const std::string ERR = "ERR";
    const std::string KEY_NOT_FOUND = "KEY_NOT_FOUND";
    const std::string INVALID_CMD = "INVALID_CMD";

    const std::string KVTP_FMT_ERR = "KVTP_FMT_ERR";
}

#endif //SIMKV_EXEC_MESSAGE_H
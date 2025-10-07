//
// Created by HongXing on 30/9/25.
//

#include <iostream>
#include "executor.h"

#include "../inc/type.h"
#include "../kvtp/request.h"

std::vector<BYTE> Executor::execute_db(kvtp::KvtpRequest kvtp_req, Db* db) {
    std::vector<BYTE> result;

    // parse key
    KeyInfo key_info = parse_key(kvtp_req.key);

    if (key_info.typ == ValueType::STR) {
        result = this->str_executor.execute(kvtp_req, key_info, db);
    }else if (key_info.typ == ValueType::LST) {
        result = this->lst_executor.execute(kvtp_req, key_info, db);
    }else if (key_info.typ == ValueType::MAP) {

    }else if (key_info.typ == ValueType::SET) {

    }

    return result;
}

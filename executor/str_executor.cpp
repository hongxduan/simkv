//
// Created by HongXing on 7/10/25.
//

#include "str_executor.h"

#include <iostream>

#include "command.h"
#include "../kvtp/response.h"
#include "../util/crc_util.h"

/*
std::vector<BYTE> StrExecutor::execute(kvtp::KvtpRequest kvtp_req, KeyInfo key_info, Db *db) {
    if (kvtp_req.cmd == cmd::GET) {
        return get(kvtp_req, db);
    }else if (kvtp_req.cmd == cmd::SET) {
        return set(kvtp_req, db);
    }else if (kvtp_req.cmd == cmd::DEL) {
        return del(kvtp_req, db);
    }else {
        // return error command
        return get(kvtp_req, db);
    }
}


std::vector<BYTE> StrExecutor::get(kvtp::KvtpRequest kvtp_req, Db *db) {
    std::vector<BYTE> result;

    // calc page index
    auto hash = util::crc16(kvtp_req.key);
    auto index = hash % PAGE_NUM;

    std::cout << "hash:" << hash<<std::endl;
    auto value = db->get(index, kvtp_req.key);
    if (value.val == nullptr) {
        std::cout << "null" << std::endl;
        result = kvtp::encode_str_response(key_not_found_rslt);
        return result;
    }

    // to string
    std::string str_val =  *static_cast<std::string *>(value.val);

    // encode kvtp response
    result = kvtp::encode_str_response(str_val);

    // return
    return result;
}

std::vector<BYTE> StrExecutor::set(kvtp::KvtpRequest kvtp_req, Db *db) {
    std::vector<BYTE> result;

    std::string *val = new std::string();
    Value value = Value();
    value.typ = ValueType::STR;
    //value.str = std::string(kvtp_req.val.begin(), kvtp_req.val.end());
    val->assign(kvtp_req.val.begin(), kvtp_req.val.end());
    value.val = val;

    // calc page index
    auto hash = util::crc16(kvtp_req.key);
    auto index = hash % PAGE_NUM;
    std::cout << index<<std::endl;
    std::cout << kvtp_req.key << " " << *(std::string*)value.val << std::endl;
    // set db
    //db->get_pages()[index][kvtp_req.key] = value;
    db->set(index, kvtp_req.key, value);

    // encode kvtp response
    result = kvtp::encode_i32_response(1);

    // return
    return result;
}

std::vector<BYTE> StrExecutor::del(kvtp::KvtpRequest kvtp_req, Db *db) {
    std::vector<BYTE> result;

    return result;
}
*/




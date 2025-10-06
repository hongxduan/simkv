//
// Created by HongXing on 2/10/25.
//

#include <vector>
#include <iostream>
#include "db.h"

#include "key.h"

/****************************
 * Db constructor implement
 ****************************/
Db::Db() {
    uint page_num = PAGE_NUM;
    for (auto i=0 ; i < page_num; i++) {
        pages.push_back(std::map<std::string, Value>());
    }
}

std::vector<BYTE> Db::execute(std::vector<BYTE> raw_req) {
    std::vector<BYTE> result;

    // decode raw req to kvtp req
    auto kvtp_req = kvtp::decode_request(raw_req);

    // parse key
    KeyInfo key_info = parse_key(kvtp_req.key);

    // determine command
    if (kvtp_req.cmd == GET) {
        result = this->get(kvtp_req);
    } else if (kvtp_req.cmd == SET) {
        result = this->set(kvtp_req);
    } else if (kvtp_req.cmd == DEL) {
        result = this->del(kvtp_req);
    } else if (kvtp_req.cmd == KEY) {
        result = this->key(kvtp_req);
    } else {
        // response error: unknown command
    }

    return result;
}

//
// get implement
//
std::vector<BYTE> Db::get(kvtp::KvtpRequest kvtp_req) {
    std::vector<BYTE> result;

    //auto value = this->page0[kvtp_req.key];
    auto value = this->pages[0][kvtp_req.key];

    std::cout << *static_cast<std::string *>(value.val) << std::endl;

    return result;
}

//
// set implement
//
std::vector<BYTE> Db::set(kvtp::KvtpRequest kvtp_req) {
    std::vector<BYTE> result;

    std::string *val = new std::string();
    Value value = Value();
    value.typ = ValueType::STR;
    //value.str = std::string(kvtp_req.val.begin(), kvtp_req.val.end());
    val->assign(kvtp_req.val.begin(), kvtp_req.val.end());
    value.val = val;
    //this->page0[kvtp_req.key] = value;
    this->pages[0][kvtp_req.key] = value;

    return result;
}

//
// del implement
//
std::vector<BYTE> Db::del(kvtp::KvtpRequest request) {
    std::vector<BYTE> result;


    return result;
}

std::vector<BYTE> Db::key(kvtp::KvtpRequest request) {
    std::vector<BYTE> result;
    return result;
}

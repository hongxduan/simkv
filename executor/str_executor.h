//
// Created by HongXing on 7/10/25.
//

#ifndef SIMKV_STR_EXECUTOR_H
#define SIMKV_STR_EXECUTOR_H
#include <iostream>

#include "command.h"
#include "db_executor.h"
#include "exec_message.h"
#include "../db/db.h"
#include "../kvtp/request.h"
#include "../kvtp/response.h"
#include "../util/crc_util.h"

class StrExecutor : public DbExecutor {
public:
    std::vector<BYTE> execute(kvtp::KvtpRequest kvtp_req, KeyInfo key_info, Db *db) {
        if (kvtp_req.cmd == cmd::GET) {
            return get(kvtp_req, db);
        } else if (kvtp_req.cmd == cmd::SET) {
            return set(kvtp_req, db);
        } else if (kvtp_req.cmd == cmd::DEL) {
            return del(kvtp_req, db);
        } else {
            // return error command
            return kvtp::encode_err_response(execmsg::INVALID_CMD);
        }
    }

protected:
    std::vector<BYTE> get(kvtp::KvtpRequest kvtp_req, Db *db) {
        std::vector<BYTE> result;

        // calc page index
        auto hash = util::crc16(kvtp_req.key);
        auto index = hash % PAGE_NUM;

        std::cout << "hash:" << hash << std::endl;
        auto value = db->get(index, kvtp_req.key);
        if (value.val == nullptr) {
            std::cout << "null" << std::endl;
            result = kvtp::encode_err_response(execmsg::KEY_NOT_FOUND);
            return result;
        }

        // to string
        std::string str_val = *static_cast<std::string *>(value.val);

        // encode kvtp response
        result = kvtp::encode_str_response(str_val);

        // return
        return result;
    }

    std::vector<BYTE> set(kvtp::KvtpRequest kvtp_req, Db *db) {
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
        std::cout << index << std::endl;
        std::cout << kvtp_req.key << " " << *(std::string *) value.val << std::endl;
        // set db
        //db->get_pages()[index][kvtp_req.key] = value;
        db->set(index, kvtp_req.key, value);

        // encode kvtp response
        result = kvtp::encode_i32_response(1);

        // return
        return result;
    }

    std::vector<BYTE> del(kvtp::KvtpRequest kvtp_req, Db *db) {
        std::vector<BYTE> result;

        return result;
    }
};

#endif //SIMKV_STR_EXECUTOR_H

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
#include "../util/time_util.h"

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

        //
        auto page = db->get_page(kvtp_req.key);
        auto value = (*page)[kvtp_req.key];
        if (value == nullptr) {
            std::cout << "null" << std::endl;
            result = kvtp::encode_err_response(execmsg::KEY_NOT_FOUND);
            return result;
        }

        // to string
        std::string str_val = *static_cast<std::string *>(value->val);

        // encode kvtp response
        result = kvtp::encode_str_response(str_val);

        // return
        return result;
    }

    std::vector<BYTE> set(kvtp::KvtpRequest kvtp_req, Db *db) {
        std::vector<BYTE> result;

        // get page
        auto page = db->get_page(kvtp_req.key);

        // get old
        auto old = (*page)[kvtp_req.key];

        // create new Valute and set val
        std::string *val = new std::string();
        auto value = new Value();
        value->typ = ValueType::STR;
        //value.str = std::string(kvtp_req.val.begin(), kvtp_req.val.end());
        val->assign(kvtp_req.val.begin(), kvtp_req.val.end());
        value->val = val;

        // handle ttl and expiration
        // todo: make set_ttl a method

        if (kvtp_req.ttl > 0) {
            auto ttl = util::ms_now();
            ttl = ttl + kvtp_req.ttl * 1000; // to mills
            ttl = util::ms_to_based(ttl);
            value->ttl = ttl; // to based

            // delete old expiration
            if (old != nullptr) {
                db->del_expiration(kvtp_req.key, old->ttl);
            }

            // set new expiration
            db->set_expiration(kvtp_req.key, ttl);

            // notify purge expiration thread
            std::lock_guard<std::mutex> unlock(expiration_mutex);
            expiration_cv.notify_one();
        } else {
            if (kvtp_req.ttl == -1) {
                value->ttl = -1;
            } else {
                //
            }
        }

        // set db
        //db->get_page[index][kvtp_req.key] = value;
        //db->set(index, kvtp_req.key, value);

        (*page)[kvtp_req.key] = value;

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

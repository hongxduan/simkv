//
// Created by HongXing on 7/10/25.
//

#ifndef SIMKV_STR_EXECUTOR_H
#define SIMKV_STR_EXECUTOR_H
#include <iostream>

#include "command.h"
#include "db_executor.h"
#include "exec_message.h"
#include "response_msg.h"
#include "../db/db.h"
#include "../kvtp/kvtp.h"
#include "../kvtp/request.h"
#include "../kvtp/response.h"
#include "../util/string_util.h"
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

        // handle arguments
        //  -ttl
        //  -del
        if (kvtp_req.args.size() > 0) {
            for (auto arg: kvtp_req.args) {
                // get ttl
                if (util::to_upper(arg) == ARG_TTL) {
                    // if ttl is -1, then return
                    if (value->ttl == -1) {
                        return kvtp::encode_i32_response(value->ttl);
                    }
                    // else return now - ttl seconds
                    auto now = util::ms_now();
                    auto ttl = (util::based_to_ms(value->ttl) - now) / 1000;
                    // set ttl to 0 if ttl is negative and not -1
                    if (ttl < 0) ttl = 0;
                    return kvtp::encode_i32_response(ttl);
                }
                // get and delete
                else if (util::to_upper(arg) == ARG_DEL) {
                    // todo
                }
            }
        }

        if (value->val != nullptr) {
            // to string
            std::string str_val = *static_cast<std::string *>(value->val);
            // encode kvtp response
            result = kvtp::encode_str_response(str_val);
        } else {
            // todo: delete the key
            return kvtp::encode_err_response(RM_UNKNOW_ERR);
        }
        // return
        return result;
    }

    std::vector<BYTE> set(kvtp::KvtpRequest kvtp_req, Db *db) {
        std::vector<BYTE> result;

        std::string *val = nullptr;
        Value *value = nullptr;

        // get page
        auto page = db->get_page(kvtp_req.key);

        // get old
        auto old = (*page)[kvtp_req.key];

        // new key
        if (old == nullptr) {
            if (kvtp_req.val.size() == 0) {
                return kvtp::encode_err_response(RM_VALUE_REQUIRED);
            }
            val = new std::string();
            value = new Value();
            value->typ = ValueType::STR;
            val->assign(kvtp_req.val.begin(), kvtp_req.val.end());
            value->val = val;
            (*page)[kvtp_req.key] = value;
        }
        // update key
        else {
            value = old;
            if (kvtp_req.val.size() > 0) {
                val = static_cast<std::string *>(value->val);
                val->assign(kvtp_req.val.begin(), kvtp_req.val.end());
                //value->val = val;
            } else {
                // if value not provided, and ttl also not provided
                // then error
                if (kvtp_req.ttl < 0) {
                    return kvtp::encode_err_response(RM_VALUE_REQUIRED);
                }
            }
        }

        // handle ttl and expiration
        // todo: make set_ttl a method
        if (kvtp_req.ttl > 0) {
            // delete old expiration *FIRST*
            if (old != nullptr) {
                db->del_expiration(kvtp_req.key, old->ttl);
            }

            // update to new ttl
            auto ttl = util::ms_now();
            ttl = ttl + kvtp_req.ttl * 1000; // to mills
            ttl = util::ms_to_based(ttl);
            value->ttl = ttl; // to based

            // set new expiration
            db->set_expiration(kvtp_req.key, ttl);

            // notify purge expiration thread
            std::lock_guard<std::mutex> lock(db->expiration_mutex);
            db->expiration_notified = true;
            db->expiration_cv.notify_one();
        } else {
            std::cout << "req.ttl " << kvtp_req.ttl << std::endl;
            if (kvtp_req.ttl == -1) {
                value->ttl = -1;
            } else {
                //
            }
        }

        // set db
        //db->get_page[index][kvtp_req.key] = value;
        //db->set(index, kvtp_req.key, value);

        //(*page)[kvtp_req.key] = value;

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

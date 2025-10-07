//
// Created by HongXing on 7/10/25.
//

#ifndef SIMKV_LST_EXECUTOR_H
#define SIMKV_LST_EXECUTOR_H

#include <vector>

#include "command.h"
#include "db_executor.h"
#include "../kvtp/request.h"
#include "../db/db.h"


class LstExecutor : public DbExecutor {
public:
    std::vector<BYTE> execute(kvtp::KvtpRequest kvtp_req, KeyInfo key_info, Db *db) override {
        if (kvtp_req.cmd == cmd::GET) {
            return get(kvtp_req, db);
        } else if (kvtp_req.cmd == cmd::SET) {
            return set(kvtp_req, db);
        } else if (kvtp_req.cmd == cmd::DEL) {
            return del(kvtp_req, db);
        } else {
            // return error command
            return get(kvtp_req, db);
        }
    }

protected:
    std::vector<BYTE> get(kvtp::KvtpRequest kvtp_req, Db *db) override {
        std::vector<BYTE> result;

        return result;
    }

    std::vector<BYTE> set(kvtp::KvtpRequest kvtp_req, Db *db) override {
        std::vector<BYTE> result;
        return result;
    }

    std::vector<BYTE> del(kvtp::KvtpRequest kvtp_req, Db *db) override {
        std::vector<BYTE> result;
        return result;
    }
};


#endif //SIMKV_LST_EXECUTOR_H

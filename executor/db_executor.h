//
// Created by HongXing on 7/10/25.
//

#ifndef SIMKV_DB_EXECUTOR_H
#define SIMKV_DB_EXECUTOR_H

#include <vector>

#include "db_executor.h"
#include "../kvtp/request.h"
#include "../db/db.h"

class DbExecutor {
public:
    virtual std::vector<BYTE> execute(kvtp::KvtpRequest kvtp_req, KeyInfo key_info, Db *db) =0;

protected:
    virtual std::vector<BYTE> get(kvtp::KvtpRequest kvtp_req, Db *db) =0;

    virtual std::vector<BYTE> set(kvtp::KvtpRequest kvtp_req, Db *db) =0;

    virtual std::vector<BYTE> del(kvtp::KvtpRequest kvtp_req, Db *db) =0;
};


#endif //SIMKV_DB_EXECUTOR_H

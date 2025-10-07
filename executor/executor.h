//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_EXECUTOR_H
#define SIMKV_EXECUTOR_H
#include <vector>

#include "lst_executor.h"
#include "str_executor.h"
#include "../db/db.h"
#include "../inc/type.h"
#include "../kvtp/request.h"

class Executor {
public:
    //
    // execute cmd and return result
    //
    std::vector<BYTE> execute_db(kvtp::KvtpRequest kvtp_req, Db *db);

private:
    StrExecutor str_executor;
    LstExecutor lst_executor;
};

#endif //SIMKV_EXECUTOR_H

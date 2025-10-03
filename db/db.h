//
// Created by HongXing on 2/10/25.
//

#ifndef SIMKV_DB_H
#define SIMKV_DB_H

#include <map>
#include <vector>

#include "key.h"
#include "value.h"
#include "../inc/type.h"
#include "../kvtp/request.h"

#define GET "GET"
#define SET "SET"
#define DEL "DEL"
#define KEY "KEY"


class Db {
public:
    std::vector<BYTE> execute(std::vector<BYTE> raw_req);

private:
    std::map<std::string, Value> page0;
    std::vector<BYTE> get(kvtp::KvtpRequest request);

    std::vector<BYTE> set(kvtp::KvtpRequest request);

    std::vector<BYTE> del(kvtp::KvtpRequest request);

    std::vector<BYTE> key(kvtp::KvtpRequest request);
};


#endif //SIMKV_DB_H

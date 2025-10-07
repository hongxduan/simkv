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

#define PAGE_NUM 64;

//
// The types that the exeuction return
//
enum ExecResultType {
    I, // Integer 32 bit
    L, // Long 64 bit
    D, // Double
    S, // String
    LI, // List of Integer
    LL, // List of Long
    LD, // List of Double
    LS, // List of String
    M, // Map
};

class Db {
public:
    Db();

    //
    // execute command
    //
    std::vector<BYTE> execute(std::vector<BYTE> raw_req);

    void set(uint16_t index, std::string key, Value value);

    Value get(uint16_t index, std::string key);

private:
    // pages in a vector
    std::vector<std::map<std::string, Value> > pages;
};


#endif //SIMKV_DB_H

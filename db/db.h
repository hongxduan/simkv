//
// Created by HongXing on 2/10/25.
//

#ifndef SIMKV_DB_H
#define SIMKV_DB_H

#include <map>
#include <mutex>
#include <vector>
#include <set>

#include "key.h"
#include "value.h"
#include "../inc/type.h"
#include "../kvtp/request.h"

#define PAGE_NUM 64;
#define MILLS_BASE 1759894044607;

struct Expiration {
    std::string key;
    uint64_t ms;

    // override < operatior
    bool operator<(const Expiration &other) const {
        return ms < other.ms;
    }
};


class Db {
public:
    Db();

    //
    // execute command
    //
    std::vector<BYTE> execute(std::vector<BYTE> raw_req);

    //
    // set value to pages[index]
    //
    void set(uint16_t index, std::string key, Value value);

    //
    // get value from pages[index]
    //
    Value get(uint16_t index, std::string key);

private:
    // pages in a vector
    std::vector<std::map<std::string, Value> > pages;

    std::set<Expiration> expirations;
};

static bool expiration_notified;
static std::mutex expiration_mutex;
static std::condition_variable expiration_cv;
//
// purge expired keys
//
static void purge_expired(Db* db);


#endif //SIMKV_DB_H

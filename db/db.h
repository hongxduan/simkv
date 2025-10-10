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

#define PAGE_NUM 64;


struct Expiration {
    std::string key;

    ///
    /// store based mills
    int64_t ms;

    // override < operatior
    bool operator<(const Expiration &other) const {
        return ms < other.ms;
    }
};


class Db {
public:
    bool expiration_notified;
    std::mutex expiration_mutex;
    std::condition_variable expiration_cv;
    Db();

    //
    // execute command
    //
    std::vector<BYTE> execute(std::vector<BYTE> raw_req);

    //
    // set value to pages[index]
    //
    void set(uint16_t index, std::string key, Value* value);

    //
    // get value from pages[index]
    //
    Value* get(uint16_t index, std::string key);

    /// get page via index
    /// @param key
    /// @return
    std::map<std::string, Value*> *get_page(std::string key);

    /// set expiration
    /// @param key
    /// @param ms
    void set_expiration(std::string key, int64_t ms);

    /// delete expiration
    /// @param key
    /// @param ms
    void del_expiration(std::string key, int64_t ms);

    std::set<Expiration>* get_expirations();

    ///
    /// @return
    ///     next expiration
    Expiration next_expiration();

private:
    // pages in a vector
    std::vector<std::map<std::string, Value*> > pages;

    std::set<Expiration> expirations;
};

//bool expiration_notified;
//std::mutex expiration_mutex;
//std::condition_variable expiration_cv;

/// task runs on a dedicated thread to purge expired keys
/// @param db
static void purge_expired_task(Db *db);

/// do purge expired keys
/// @param db
/// @return
///     the first based ms greater than now, i.e. the one on the top of tree
///     the purge task will sleep base on this return value
static int64_t purge_expired_keys(Db *db);

#endif //SIMKV_DB_H

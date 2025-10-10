//
// Created by HongXing on 2/10/25.
//

#include <vector>
#include <iostream>
#include <condition_variable>
#include <chrono>
#include <mutex>
#include <thread>

#include "db.h"

#include "key.h"
#include "../kvtp/response.h"
#include "../util/crc_util.h"
#include "../util/string_util.h"
#include "../util/time_util.h"

//
// Db constructor implement
//
Db::Db() {
    uint page_num = PAGE_NUM;
    for (auto i = 0; i < page_num; i++) {
        pages.push_back(std::map<std::string, Value *>());
    }

    std::thread purge_expiration_thread(purge_expired_task, this);
    purge_expiration_thread.detach();
}

void Db::set(uint16_t index, std::string key, Value *value) {
    // unlock and notify
    std::lock_guard<std::mutex> lock(expiration_mutex);
    expiration_cv.notify_one();

    this->pages[index][key] = value;
}

Value *Db::get(uint16_t index, std::string key) {
    return this->pages[index][key];
}

std::map<std::string, Value *> *Db::get_page(std::string key) {
    auto hash = util::crc16(key);
    auto index = hash % PAGE_NUM;
    return &this->pages[index];
}

void Db::set_expiration(std::string key, int64_t ms) {
    this->expirations.insert({key, ms});
}

void Db::del_expiration(std::string key, int64_t ms) {
    this->expirations.erase({key, ms});
}

std::set<Expiration> *Db::get_expirations() {
    return &expirations;
}


Expiration Db::next_expiration() {
    auto first = this->expirations.begin();
    return *first;
}

void purge_expired_task(Db *db) {
    while (true) {
        auto ms = purge_expired_keys(db);
        ms = util::based_to_ms(ms);
        auto now = util::ms_now();

        // wait until next expiration
        if (ms - now > 0) {
            std::cout << "blocking..." << std::endl;
            std::unique_lock<std::mutex> lock(db->expiration_mutex);
            db->expiration_cv.wait_for(lock,
                                   std::chrono::milliseconds(ms - now),
                                   [db] { return db->expiration_notified; });
            std::cout << "unblocked." << std::endl;
        }
        db->expiration_notified = false; // reset notified to false
        /*
        if (expiration_notified) {
            std::unique_lock<std::mutex> lock(expiration_mutex);
            std::cout << "notified" << std::endl;
            expiration_notified = false;
        }else {
            std::this_thread::sleep_for(std::chrono::seconds(5));
            std::cout << "wakeup" << std::endl;
        }*/
        //expiration_cv.wait_for(lock, std::chrono::seconds(3));
    }
}

int64_t purge_expired_keys(Db *db) {
    auto now = util::ms_now();
    auto expirations = db->get_expirations();
    //for (auto it = expirations.begin(); it != db->get_expirations().end(); ++it) {
    auto it = expirations->begin();
    while (it != expirations->end()) {
        if (it->key == "") {
            break;
        }
        if (util::based_to_ms(it->ms) < now) {
            std::cout << "purge_expired_keys - purging: " << it->key << std::endl;

            // expired
            auto page = db->get_page(it->key);

            // clear memory
            auto value = (*page)[it->key];
            if (value == nullptr) {
                expirations->erase(it++);
                continue;
            }
            if (value->typ == ValueType::STR) {
                delete static_cast<std::string *>(value->val);
            } else if (value->typ == ValueType::LST) {
                delete[] static_cast<std::list<std::vector<BYTE> > *>(value->val);
            }

            value->val = nullptr;
            delete value;
            value = nullptr;

            // erase the item from page
            page->erase(it->key);

            // erase expiration
            expirations->erase(it++);
            std::cout << "purge_expired_keys - purged" << std::endl;
        } else {
            return it->ms;
        }
    }
    return 0;
}

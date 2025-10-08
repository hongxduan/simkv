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
#include "../util/string_util.h"

//
// Db constructor implement
//
Db::Db() {
    uint page_num = PAGE_NUM;
    for (auto i = 0; i < page_num; i++) {
        pages.push_back(std::map<std::string, Value>());
    }

    std::thread purge_expiration_thread(purge_expired, this);
    purge_expiration_thread.detach();
}

void Db::set(uint16_t index, std::string key, Value value) {

    // unlock and notify
    std::lock_guard<std::mutex> lock(expiration_mutex);
    expiration_notified = true;
    expiration_cv.notify_one();

    this->pages[index][key] = value;
}

Value Db::get(uint16_t index, std::string key) {
    return this->pages[index][key];
}

void purge_expired(Db* db) {
    while (true) {
        std::unique_lock<std::mutex> lock(expiration_mutex);
        std::cout << "notified" << std::endl;
        expiration_notified = false;
        //std::this_thread::sleep_for(std::chrono::seconds(5));
        expiration_cv.wait_for(lock, std::chrono::seconds(5));
        std::cout << "wakeup" << std::endl;

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


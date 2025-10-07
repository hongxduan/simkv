//
// Created by HongXing on 2/10/25.
//

#include <vector>
#include <iostream>
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
}

void Db::set(uint16_t index, std::string key, Value value) {
    this->pages[index][key] = value;
}

Value Db::get(uint16_t index, std::string key) {
    return this->pages[index][key];
}


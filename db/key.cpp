//
// Created by HongXing on 2/10/25.
//

#include <regex>
#include "key.h"

#include <iostream>
#include <ostream>

//
// parseKey implement
//
KeyInfo parse_key(std::string raw_key) {
    KeyInfo info;
    std::smatch match;
    if (std::regex_match(raw_key, match, lst_key_reg)) {
        info.typ = ValueType::LST;
        if (match.size() == 3) {
            info.key = match[1].str();
            info.skey = match[2].str();
        } else {
            //todo: handle invalid key
        }
    } else if (std::regex_match(raw_key, match, map_key_reg)) {
        info.typ = ValueType::MAP;
    } else if (std::regex_match(raw_key, match, set_key_reg)) {
        info.typ = ValueType::SET;
    }
    // default is String type
    else {
        info.typ = ValueType::STR;
    }

    return info;
}

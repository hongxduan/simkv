//
// Created by HongXing on 2/10/25.
//
// Key Header file
// Including different key regex pattern for parse key
//

#ifndef SIMKV_KEY_H
#define SIMKV_KEY_H

#include <regex>
#include "value.h"


/*****************************************************************************
 set users[0] xxx     ... left push
 set users[-1] xxx    ... right push
 set users[5]  xxx    ... insert at index 5
 set users[&5] xxx    ... replace value at index 5
 get users[0]         ... get the first
 get users[-1]        ... get the last
 get users[#]         ... get len
 get users[&abc]      ... get index of value abc
 get users[3..5]      ... return the values from index 3 to 4; right exclusive
 get users[3..]       ... return the values from index 3 to the last
 get users[..]        ... return all values
******************************************************************************/
//#define LST_KEY_PATTERN "(?<key>.+)\[(?<skey>(-?[0-9]+|-?[0-9]+\.\.-?[0-9]+|#|\$-?[0-9]+|\&.+)+)\]$"
#define LST_KEY_PATTERN R"((.+)\[(-?[0-9]+|-?[0-9]+\.\.-?[0-9]+|#|\$-?[0-9]+|\&.+)\]$)"

//
// set user_age{jerry} 3
// set user_age{tom jerry} 5 3
// get user_age{tom}
// get user_age{tom jerry}
//
//#define MAP_KEY_PATTERN "(?<key>.+)\{(?<skey>[^\{\}]+)\}$"
#define MAP_KEY_PATTERN R"((.+)\{([^\{\}]+)\}$)"

//
//
//
//#define SET_KEY_PATTERN "(?<key>.+)<(?<skey>[^<>]+)>$"
#define SET_KEY_PATTERN R"((.+)<([^<>]+)>$)"

const std::regex lst_key_reg(LST_KEY_PATTERN);
const std::regex map_key_reg(MAP_KEY_PATTERN);
const std::regex set_key_reg(SET_KEY_PATTERN);

//
// Key type and Key, result of parse key
//
struct KeyInfo {
    ValueType typ; // the value type of the key
    std::string key; // the key
    std::string skey; // the sub key
};

//
// parse raw key to KeyInfo
//
KeyInfo parse_key(std::string raw_key);

#endif //SIMKV_KEY_H

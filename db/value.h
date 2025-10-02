//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_VALUE_H
#define SIMKV_VALUE_H
#include <vector>

//
// Value types
//
enum ValueType {
    STRING,
    LIST,
    MAP,
    HASH,
    SET
};

//
// Value is one of the types in the union object
//
union ValueUnion {
    std::vector<u_int8_t> StrVal;
    std::vector<std::vector<uint8_t> > LstVal;
};

//
// Value
//
struct Value {
    ValueType type;
    ValueUnion val;
};


#endif //SIMKV_VALUE_H

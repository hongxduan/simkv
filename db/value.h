//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_VALUE_H
#define SIMKV_VALUE_H
#include <list>
#include <vector>

#include "../inc/type.h"

//
// Value types
//
enum ValueType {
    STR,
    LST,
    MAP,
    SET
};

//
// Value is one of the types in the union object
// have problem???
//
union ValueUnion {
    //std::vector<BYTE> str;
    //std::vector<std::vector<BYTE> > lst;
    ValueUnion() {
    }

    std::string str;
    std::list<std::vector<BYTE> > lst;

    ~ValueUnion() {
    }
};

//
// Value
//
struct Value {
    ValueType typ;
    //
    // str: *string
    // lst: *list<std::vector<BYTE>>
    //
    void *val;

    ///
    /// store based ms
    ///
    int64_t ttl = -1;
    //ValueUnion val;
};


#endif //SIMKV_VALUE_H

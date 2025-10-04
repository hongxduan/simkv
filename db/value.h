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
//
union ValueUnion {
    //std::vector<BYTE> str;
    //std::vector<std::vector<BYTE> > lst;
    ValueUnion (){}
    char* str;
    std::list<char*> lst;

    ~ValueUnion() {
        delete str;
        for (auto p: lst) {
            delete p;
        }
    }
};

//
// Value
//
struct Value {
    ValueType typ;
    std::string str;
    std::list<std::vector<BYTE>> lst;
    //ValueUnion val;

};


#endif //SIMKV_VALUE_H

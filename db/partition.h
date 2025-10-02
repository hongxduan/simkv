//
// Created by HongXing on 29/9/25.
//

#ifndef SIMKV_PARTITION_H
#define SIMKV_PARTITION_H


#include <map>
#include <vector>

#include "value.h"

//
//
//
struct Partition {
    // page 0-7
    std::map<std::vector<u_int8_t>, Value> Page0;
    std::map<std::vector<u_int8_t>, Value> Page1;
    std::map<std::vector<u_int8_t>, Value> Page2;
    std::map<std::vector<u_int8_t>, Value> Page3;
    std::map<std::vector<u_int8_t>, Value> Page4;
    std::map<std::vector<u_int8_t>, Value> Page5;
    std::map<std::vector<u_int8_t>, Value> Page6;
    std::map<std::vector<u_int8_t>, Value> Page7;

    // page 8-15
    std::map<std::vector<u_int8_t>, Value> Page8;
    std::map<std::vector<u_int8_t>, Value> Page9;
    std::map<std::vector<u_int8_t>, Value> Page10;
    std::map<std::vector<u_int8_t>, Value> Page11;
    std::map<std::vector<u_int8_t>, Value> Page12;
    std::map<std::vector<u_int8_t>, Value> Page13;
    std::map<std::vector<u_int8_t>, Value> Page14;
    std::map<std::vector<u_int8_t>, Value> Page15;

    // page 16-23
    std::map<std::vector<u_int8_t>, Value> Page16;
    std::map<std::vector<u_int8_t>, Value> Page17;
    std::map<std::vector<u_int8_t>, Value> Page18;
    std::map<std::vector<u_int8_t>, Value> Page19;
    std::map<std::vector<u_int8_t>, Value> Page20;
    std::map<std::vector<u_int8_t>, Value> Page21;
    std::map<std::vector<u_int8_t>, Value> Page22;
    std::map<std::vector<u_int8_t>, Value> Page23;

    // page 24-31
    std::map<std::vector<u_int8_t>, Value> Page24;
    std::map<std::vector<u_int8_t>, Value> Page25;
    std::map<std::vector<u_int8_t>, Value> Page26;
    std::map<std::vector<u_int8_t>, Value> Page27;
    std::map<std::vector<u_int8_t>, Value> Page28;
    std::map<std::vector<u_int8_t>, Value> Page29;
    std::map<std::vector<u_int8_t>, Value> Page30;
    std::map<std::vector<u_int8_t>, Value> Page31;

    // page 32-39
    std::map<std::vector<u_int8_t>, Value> Page32;
    std::map<std::vector<u_int8_t>, Value> Page33;
    std::map<std::vector<u_int8_t>, Value> Page34;
    std::map<std::vector<u_int8_t>, Value> Page35;
    std::map<std::vector<u_int8_t>, Value> Page36;
    std::map<std::vector<u_int8_t>, Value> Page37;
    std::map<std::vector<u_int8_t>, Value> Page38;
    std::map<std::vector<u_int8_t>, Value> Page39;

    // page 40-47
    std::map<std::vector<u_int8_t>, Value> Page40;
    std::map<std::vector<u_int8_t>, Value> Page41;
    std::map<std::vector<u_int8_t>, Value> Page42;
    std::map<std::vector<u_int8_t>, Value> Page43;
    std::map<std::vector<u_int8_t>, Value> Page44;
    std::map<std::vector<u_int8_t>, Value> Page45;
    std::map<std::vector<u_int8_t>, Value> Page46;
    std::map<std::vector<u_int8_t>, Value> Page47;

    // page 48-55
    std::map<std::vector<u_int8_t>, Value> Page48;
    std::map<std::vector<u_int8_t>, Value> Page49;
    std::map<std::vector<u_int8_t>, Value> Page50;
    std::map<std::vector<u_int8_t>, Value> Page51;
    std::map<std::vector<u_int8_t>, Value> Page52;
    std::map<std::vector<u_int8_t>, Value> Page53;
    std::map<std::vector<u_int8_t>, Value> Page54;
    std::map<std::vector<u_int8_t>, Value> Page55;

    // page 56-63
    std::map<std::vector<u_int8_t>, Value> Page56;
    std::map<std::vector<u_int8_t>, Value> Page57;
    std::map<std::vector<u_int8_t>, Value> Page58;
    std::map<std::vector<u_int8_t>, Value> Page59;
    std::map<std::vector<u_int8_t>, Value> Page60;
    std::map<std::vector<u_int8_t>, Value> Page61;
    std::map<std::vector<u_int8_t>, Value> Page62;
    std::map<std::vector<u_int8_t>, Value> Page63;
};


#endif //SIMKV_PARTITION_H

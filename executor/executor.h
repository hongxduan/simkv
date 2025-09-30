//
// Created by HongXing on 30/9/25.
//

#ifndef SIMKV_EXECUTOR_H
#define SIMKV_EXECUTOR_H
#include <vector>

/*
 * parse command
 */
void parseCmd(std::vector<uint8_t> cmd);

/*
 * execute cmd and return result
 */
std::vector<uint8_t> execute(std::vector<uint8_t> cmd);


#endif //SIMKV_EXECUTOR_H

//
// Created by HongXing on 8/10/25.
//

#include "time_util.h"

#include <chrono>

int64_t util::ms_now() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::system_clock::now().time_since_epoch()).count();
}

int64_t util::ms_to_based(int64_t ms) {
    return ms - (int64_t) MILLS_BASE;
}

int64_t util::based_to_ms(int64_t based) {
    return based + (int64_t)MILLS_BASE;
}

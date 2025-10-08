//
// Created by HongXing on 8/10/25.
//

#ifndef SIMKV_TIME_UTIL_H
#define SIMKV_TIME_UTIL_H
#include <cstdint>

///
/// to reduce the scale of stored ttl, real ttl will minus the base value
/// will change to a more new value, but after release first version, DO NOT change this
///
/// expiration use based ms
#define MILLS_BASE 1759915811194;

namespace util {
    ///
    /// @return
    ///     epoch mills
    int64_t ms_now();

    /// get based ms by minus the base
    /// @return
    int64_t ms_to_based(int64_t ms);

    /// get the original ms by plus back the base
    /// @return
    int64_t based_to_ms(int64_t based);
}


#endif //SIMKV_TIME_UTIL_H

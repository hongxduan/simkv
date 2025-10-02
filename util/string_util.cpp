//
// Created by HongXing on 29/9/25.
//

#include <iostream>

#include "string_util.h"

/* Trim string left */
std::string util::trim_left(const std::string &str) {
    size_t start = str.find_first_not_of(" \t\n");
    return (start == std::string::npos) ? "" : str.substr(start);
}

/* Trim string right */
std::string util::trim_right(const std::string &str) {
    size_t end = str.find_last_not_of(" \t\n");
    return (end == std::string::npos) ? "" : str.substr(0, end + 1);
}

/* Trim string left and right */
std::string util::trim(const std::string &str) {
    return trim_right(trim_left(str));
}

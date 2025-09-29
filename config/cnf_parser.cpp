//
// Created by HongXing on 28/9/25.
//

#include <istream>
#include <map>
#include <string>
#include <fstream>
#include <algorithm>

#include "cnf_parser.h"

#include "config.h"
#include "../utility/string_utility.h"

#define CONFIG_FILE_NAME "simkv_sample.cnf"

/* Print config */
void print(std::map<std::string, std::string> map) {
    for (auto it = map.begin(); it != map.end(); ++it) {
        std::cout << it->first << "=" << it->second << std::endl;
    }
}

/* Validate config values
 * For example: the server must be a valid IP address and port number
 */
Config validate(std::map<std::string, std::string> map) {
    auto config = Config();
    for (auto it = map.begin(); it != map.end(); ++it) {
        if (it->first == "server") {
            auto delimiterPos = it->second.find(":");
            auto host = trim(it->second.substr(0, delimiterPos));
            auto port = trim(it->second.substr(delimiterPos + 1));

            // validate host is IP address

            // validate port is unsigned number
            config.setHost(host);

        }
    }
    return config;
}

std::map<std::string, std::string> parse() {
    std::map<std::string, std::string> map = {};
    if (std::ifstream cFile(CONFIG_FILE_NAME); cFile.is_open()) {
        std::string line;
        while (getline(cFile, line)) {
            line.erase(std::remove_if(line.begin(), line.end(), isspace),
                       line.end());
            if (line.empty() || line[0] == '#') {
                continue;
            }
            auto delimiterPos = line.find("=");
            auto name = trim(line.substr(0, delimiterPos));
            auto value = trim(line.substr(delimiterPos + 1));
            map[name] = value;
        }
    } else {
        std::cerr << "Couldn't open config file " << CONFIG_FILE_NAME << ".\n";
    }

    // Validate config
    auto config = validate(map);

    // Print config values
    print(map);

    return map;
}

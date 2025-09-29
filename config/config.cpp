//
// Created by HongXing on 29/9/25.
//

#include <fstream>
#include "config.h"
#include "../utility/string_utility.h"

void Config::setHost(std::string host) {
    this->host = host;
}

void Config::setPort(uint16_t port) {
    this->port = port;
}

std::string Config::getHost() {
    return this->host;
}

uint16_t Config::getPort() {
    return this->port;
}

void Config::validate(std::map<std::string, std::string> map) {
    for (auto it = map.begin(); it != map.end(); ++it) {
        if (it->first == "server") {
            auto delimiterPos = it->second.find(":");
            auto host = trim(it->second.substr(0, delimiterPos));
            auto port = trim(it->second.substr(delimiterPos + 1));

            // validate host is IP address

            // validate port is unsigned number
            setHost(host);
        }
    }
}

void Config::print(std::map<std::string, std::string> map) {
    for (auto it = map.begin(); it != map.end(); ++it) {
        std::cout << it->first << "=" << it->second << std::endl;
    }
}


void Config::parse() {
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
    validate(map);

    // Print config values
    print(map);
}

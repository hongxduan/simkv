//
// Created by HongXing on 29/9/25.
//

#ifndef SIMKV_CONFIG_H
#define SIMKV_CONFIG_H

#include <iostream>
#include <map>

#define CONFIG_FILE_NAME "simkv.cnf"

class Config {
    private:
    std::string host;
    uint16_t port;
    void print(std::map<std::string, std::string> map) ;
    void validate(std::map<std::string, std::string> map) ;

    public:
    void parse();
    void setHost(std::string host);
    void setPort(uint16_t port);
    std::string getHost();
    uint16_t getPort();
};


#endif //SIMKV_CONFIG_H
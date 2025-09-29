//
// Created by HongXing on 28/9/25.
//

#ifndef SIMKV_SERVER_H
#define SIMKV_SERVER_H


#include "../config/config.h"

class Server {

    private:
    Config config;

    public:
    Server(Config config);
    void create();
};


#endif //SIMKV_SERVER_H
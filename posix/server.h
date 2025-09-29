//
// Created by HongXing on 28/9/25.
//

#ifndef SIMKV_SERVER_H
#define SIMKV_SERVER_H


#include "../config/config.h"

#define BUF_SIZE 1024;
#define MAX_CONNECTIONS 1000;

class Server {

    private:
    Config config;
    void startSelectServer();
    void startEpollServer();

    public:
    Server(Config config);
    void create();
};


#endif //SIMKV_SERVER_H
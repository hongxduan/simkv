//
// Created by HongXing on 28/9/25.
//

#ifndef SIMKV_SERVER_H
#define SIMKV_SERVER_H

#include <vector>
#include "../config/config.h"

#define BUF_SIZE 1024;
#define MAX_CONNECTIONS 1000;

/*
 *
 *
 *
 */
class Server {

    private:
    Config config;
    std::vector<int> clientList; // for storing all the client fd
    void startSelectServer();
    void startEpollServer();
    std::vector<uint8_t> handler(int fd, int i);

    public:
    Server(Config config);
    void start();
};


#endif //SIMKV_SERVER_H
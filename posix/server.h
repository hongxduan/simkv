//
// Created by HongXing on 28/9/25.
//

#ifndef SIMKV_SERVER_H
#define SIMKV_SERVER_H

#include <vector>
#include "../config/config.h"
#include "../db/db.h"

#define BUF_SIZE 1024;
#define MAX_CONNECTIONS 1000;

/*
 *
 *
 *
 */
class Server {

    private:
    Db *db;
    Config config;
    std::vector<int> clients; // select: for storing all the client fd
    void start_select_server();
    void start_epoll_server();
    std::vector<uint8_t> handler(int fd, int i);

    public:
    Server(Config config);
    void start();
};


#endif //SIMKV_SERVER_H
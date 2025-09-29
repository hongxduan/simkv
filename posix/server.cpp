//
// Created by HongXing on 28/9/25.
//
#include <iostream>
#include "server.h"
#include "../config/config.h"

Server::Server(Config config) {
    this->config = config;
}


void Server::create() {
    std::cout << "Server starting" << std::endl;

    std::cout << "Host: " << config.getHost() << " " << "Port: " << config.getPort() << std::endl;




    std::cout << "Server started" << std::endl;
}

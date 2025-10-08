//
// Created by HongXing on 28/9/25.
//
#include "server.h"

#include <iostream>
#include <vector>

#include "../executor/executor.h"
#include "../util/byte_util.h"

#ifdef __APPLE__
#include <sys/select.h>
#endif
#ifdef __linux__
#include <sys/epoll.h>
#endif

#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include <poll.h>
#include <iostream>
#include <string.h>
#include <vector>
#include <errno.h>

#include "server.h"
#include "../config/config.h"
#include "../executor/command.h"


Server::Server(Config config) {
    this->config = config;
    this->db = new Db();
    this->executor = new Executor();
}

/*
 * Multiplexing implement use select
 **/
void Server::start_select_server() {
    /*
     * create server socket
     */
    int32_t serverFD = socket(AF_INET, SOCK_STREAM, 0);
    if (serverFD < 0) {
        std::cerr << "socket created failed.";
        exit(EXIT_FAILURE);
    }
    /*
     * setting serverFD to allow multiple connection
     */
    int opt = 1;
    if (setsockopt(serverFD, SOL_SOCKET, SO_REUSEADDR, (char *) &opt, sizeof(opt)) < 0) {
        std::cerr << "setsockopt error\n";
        exit(EXIT_FAILURE);
    }

    /*
     * set server address
     */
    struct sockaddr_in serverAddr;
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(config.getPort());
    inet_pton(AF_INET, config.getHost().c_str(), &serverAddr.sin_addr);
    if (bind(serverFD, (struct sockaddr *) &serverAddr, sizeof(serverAddr)) < 0) {
        std::cerr << "bind failed.\n";
        perror("bind failed");
        exit(EXIT_FAILURE);
    }

    /*
     * listen
     */
    if (listen(serverFD, 10) < 0) {
        std::cerr << "listen failed.\n";
        exit(EXIT_FAILURE);
    } else {
        std::cout << "server is listening...\n";
    }

    fd_set readfds;
    size_t valread;
    int maxfd;
    int sd = 0;
    int activity;
    //std::vector<int> clientList; // for storing all the client fd
    while (true) {
        FD_ZERO(&readfds);
        FD_SET(serverFD, &readfds);
        maxfd = serverFD;
        for (auto sd: clients) {
            FD_SET(sd, &readfds);
            if (sd > maxfd) {
                maxfd = sd;
            }
        }
        if (sd > maxfd) {
            maxfd = sd;
        }
        activity = select(maxfd + 1, &readfds, NULL, NULL, NULL);
        if (activity < 0) {
            std::cerr << "select failed.\n";
            continue;
        }

        /*
         * if something happend on serverFD, then it means new connection request
         */
        struct sockaddr_in clientAddr;
        if (FD_ISSET(serverFD, &readfds)) {
            auto clientFD = accept(serverFD, (struct sockaddr *) &clientAddr, (socklen_t *) &clientAddr);
            if (clientFD < 0) {
                std::cerr << "accept failed.\n";
                perror("accept failed");
                continue;
            }
            // add clientFD to list
            clients.push_back(clientFD);
            std::cout << "new client connectd: " << std::endl;
            std::cout << "new connection, socket fd is " << clientFD << ", ip is: "
                    << inet_ntoa(clientAddr.sin_addr) << " port is: " << ntohs(clientAddr.sin_port) << std::endl;
        }
        /*
         * else, some io operation on some socket
         */
        auto bufsize = BUF_SIZE;
        char buffer[bufsize + 1];

        for (int i = 0; i < clients.size(); i++) {
            sd = clients[i];
            if (FD_ISSET(sd, &readfds)) {
                handler(sd, i);
                /*
                valread = read(sd, buffer, bufsize);
                // check if client disconnected
                if (valread == 0) {
                    std::cerr << "client disconnected.\n";
                    getpeername(sd, (struct sockaddr *) &clientAddr, (socklen_t *) &clientAddr);
                    std::cout << "client disconnected, ip is: " << inet_ntoa(clientAddr.sin_addr)
                            << "port is: " << ntohs(clientAddr.sin_port) << std::endl;
                    close(sd);
                    clientList.erase(clientList.begin() + i);
                } else {
                    std::cerr << "message from client: " << buffer << "\n";
                }
                */
            }
        }
    }
}


void Server::start_epoll_server() {
}

/*
 * handle client socket
 */
std::vector<uint8_t> Server::handler(int fd, int i) {
    std::vector<uint8_t> response;

    //
    // read 4 bytes, to get the message length
    // the bytes are little endian order
    //
    uint8_t len_buf[4] = {0, 0, 0, 0};
    size_t n = read(fd, len_buf, 4);
    if (n == 0) {
        std::cerr << "client disconnected.\n";
        close(fd);
        clients.erase(clients.begin() + i);
        return response;
    }

    uint32_t len = util::bytes_to_uint32(len_buf);
    //std::cout << "message len:" << len << std::endl;

    auto bufsize = BUF_SIZE;
    char buffer[bufsize];
    struct sockaddr_in clientAddr;
    std::vector<uint8_t> message; // the whole message from client

    //
    // while total read bytes less than message length, then keep reading
    //
    n = 0;
    size_t total_n = 0;
    do {
        memset(buffer, 0, bufsize);
        n = read(fd, buffer, bufsize);
        if (n == 0) {
            std::cerr << "client disconnected.\n";
            getpeername(fd, (struct sockaddr *) &clientAddr, (socklen_t *) &clientAddr);
            std::cout << "client disconnected, ip is: " << inet_ntoa(clientAddr.sin_addr)
                    << "port is: " << ntohs(clientAddr.sin_port) << std::endl;
            close(fd);
            clients.erase(clients.begin() + i);
        }
        total_n += n;
        message.insert(message.end(), buffer, buffer + n);
    } while (total_n < len);

    while (message[message.size() - 1] == '\n') {
        std::cout << "\0 removed" << std::endl;
        message.pop_back();
    }

    // decode raw req to kvtp req
    auto kvtp_req = kvtp::decode_request(message);
    if (kvtp_req.cmd == cmd::GET || kvtp_req.cmd == cmd::SET || kvtp_req.cmd == cmd::DEL || kvtp_req.cmd == cmd::KEY) {
        response = executor->execute_db(kvtp_req, db);
    } else if (kvtp_req.cmd == cmd::CLUSTER){
        // execute non-db commands
    }else {
        response = kvtp::encode_err_response(execmsg::INVALID_CMD);
    }

    send(fd, response.data(), response.size(), 0);
    return response;
}

void Server::start() {
    std::cout << "Server starting" << std::endl;

    std::cout << "Host: " << config.getHost() << " " << "Port: " << config.getPort() << std::endl;

#ifdef __APPLE__
    start_select_server();
#endif
# ifdef __linux__
    start_epoll_server();
#endif

    std::cout << "Server started" << std::endl;
}

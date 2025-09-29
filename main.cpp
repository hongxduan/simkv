#include <iostream>

#include "config/cnf_parser.h"
#include "posix/server.h"
#include "config/config.h"
// TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
int main() {

    // Parse config
    auto config = Config();
    config.parse();
    auto *tcp_server = new Server(config);
    tcp_server->create();

    // Clear
    delete tcp_server;

    return 0;
    // TIP See CLion help at <a href="https://www.jetbrains.com/help/clion/">jetbrains.com/help/clion/</a>. Also, you can try interactive lessons for CLion by selecting 'Help | Learn IDE Features' from the main menu.
}
//
// Created by HongXing on 28/9/25.
//

#include <istream>
#include <map>
#include <string>
#include <fstream>
#include <algorithm>

#include "cnf_parser.h"

std::map<std::string, std::string> parse() {
    std::map<std::string, std::string> map={};
    std::ifstream cFile ("simkv.cnf");
    if (cFile.is_open())
    {
        std::string line;
        while(getline(cFile, line))
        {
            line.erase(std::remove_if(line.begin(), line.end(), isspace),
                                 line.end());
            if( line.empty() || line[0] == '#' )
            {
                continue;
            }
            auto delimiterPos = line.find("=");
            auto name = line.substr(0, delimiterPos);
            auto value = line.substr(delimiterPos + 1);
            map[name] = value;
            std::cout << name << " " << value << '\n';
        }
    }
    else
    {
        std::cerr << "Couldn't open config file for reading.\n";
    }

    return map;
}

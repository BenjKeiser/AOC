#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

#define FACTOR 17

unsigned char Elves::get_hash(std::string str)
{
    unsigned char hash = 0;
    for(int i = 0; i < str.length(); i++)
    {
        hash += str[i];
        hash *= FACTOR;
    }
    return hash;
}


uint64_t Elves::get_hash_sum()
{
    uint64_t sum = 0;
    unsigned char hash = 0;
    for(auto & s: steps)
    {
        hash = get_hash(s);
        sum += hash;

        std::cout << s << ": " << (int)hash << std::endl;
    }

    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line, ',')) 
        {
            if(line.length() > 0)
            {
                steps.push_back(line);
            }
        }
        
        file.close();
    }
}


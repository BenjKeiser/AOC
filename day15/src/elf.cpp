#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

#define FACTOR 17

void print_map(const std::unordered_map <std::string, int>& m)
{
    // Iterate using C++17 facilities
    for (const auto& [key, value] : m)
        std::cout << '[' << key << "] = " << value << "; ";
 
    std::cout << std::endl;
}

void print_boxes(const std::vector<std::unordered_map<std::string, int>> & boxes)
{
    for(auto & b : boxes)
    {
        if(b.size() > 0)
        {
            print_map(b);
        }
    }
    std::cout << std::endl;
}

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

        //std::cout << s << ": " << (int)hash << std::endl;
    }

    return sum;
}

uint64_t Elves::get_focus_power()
{
    uint64_t sum = 0;
    unsigned char hash = 0;
    size_t pos = 0;
    uint64_t val = 0;

    for(auto & s: steps)
    {
        pos = s.find('=');
        if(pos != std::string::npos)
        {
            hash = get_hash(s.substr(0, pos));
            boxes[hash][s.substr(0, pos)] = std::stoi(s.substr(pos+1));

        }
        else
        {
            pos = s.find('-');
            hash = get_hash(s.substr(0, pos));
            boxes[hash].erase(s.substr(0, pos));
        }        
        print_boxes(boxes);
    }

    for(int i = 0; i < boxes.size(); i++)
    {
        int k = 0;
        for (auto iter = boxes[i].begin(); iter != boxes[i].end(); ++iter) 
        {
            val = (i+1) * (boxes[i].size() - k) * iter->second;

            //std::cout << iter->first << ": " << iter->second << " = " << val << std::endl;
            sum += val;
            k++;
        }
    }

    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::unordered_map<std::string, int> box;

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

    for(int i = 0; i < 256; i++)
    {
        boxes.push_back(box);
    }
}


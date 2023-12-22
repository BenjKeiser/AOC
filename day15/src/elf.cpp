#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

#define FACTOR 17

void print_box(const std::vector <lens_t> & box)
{
    for (const auto & l : box)
    {
        std::cout << '[' << l.label << " " << l.focal_length << "] ";
    }
    std::cout << std::endl;
}

void print_boxes(const std::vector<std::vector<lens_t>> & boxes)
{
    for(auto & b : boxes)
    {
        if(b.size() > 0)
        {
            print_box(b);
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
    }

    return sum;
}

uint64_t Elves::get_focus_power()
{
    uint64_t sum = 0;
    unsigned char hash = 0;
    size_t pos = 0;
    uint64_t val = 0;

    bool found = false;

    for(auto & s: steps)
    {
        found = false;
        pos = s.find('=');
        
        if(pos != std::string::npos)
        {
            hash = get_hash(s.substr(0, pos));
            for(int i = 0; i < boxes[hash].size(); i++)
            {
                if(boxes[hash][i].label == s.substr(0, pos))
                {
                    boxes[hash][i].focal_length = std::stoi(s.substr(pos+1));
                    found = true;
                    break;
                }
            }

            if(!found)
            {
                boxes[hash].push_back({s.substr(0, pos), std::stoi(s.substr(pos+1))});
            }
        }
        else
        {
            pos = s.find('-');
            hash = get_hash(s.substr(0, pos));
                
            auto it = boxes[hash].begin();

            while ( it != boxes[hash].end())
            {
                if (it->label == s.substr(0, pos))
                {
                    it = boxes[hash].erase(it);
                }
                else
                {
                    it++;
                }
            }
        }        
        //print_boxes(boxes);
    }

    print_boxes(boxes);

    for(int i = 0; i < boxes.size(); i++)
    {
        for (int k = 0; k < boxes[i].size(); k++) 
        {
            val = (i+1) * (k+1) * boxes[i][k].focal_length;

            sum += val;
        }
    }

    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<lens_t> box;

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


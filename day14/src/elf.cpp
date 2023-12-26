#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

void print_dish(std::vector <std::string> * dish)
{
    for(auto & l : *dish)
    {
        std::cout << l << std::endl;
    }
    std::cout << std::endl;
}

uint64_t calc_load(std::vector <std::string> * dish)
{
    uint64_t load = 0;
    int factor = dish->size();
    for(auto & r : *dish)
    {
        load += (factor * std::count(r.begin(), r.end(), 'O'));
        factor--;
    }
    return load;
}

std::vector<std::string> tilt_north(std::vector <std::string> * dish)
{
    std::vector<std::string> tilted_dish = *dish;

    int tilt = 0;

    for(int i = 1; i < tilted_dish.size(); i++)
    {
        for(int k = 0; k < tilted_dish[i].length(); k++)
        {
            if(tilted_dish[i][k] == 'O')
            {
                tilt = 0;
                for(int z = 1; i-z >= 0; z++)
                {
                    if(tilted_dish[i-z][k] == '.')
                    {
                        tilt++;
                    }
                    else
                    {
                        break;
                    }
                }
                tilted_dish[i][k] = '.';
                tilted_dish[i - tilt][k] = 'O';
            }
        }
    }
    return tilted_dish;
}

uint64_t Elves::get_load()
{
    uint64_t load = 0;

    std::vector<std::string> tilted_dish = tilt_north(&dish);

    load = calc_load(&tilted_dish);

    return load;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    int nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            dish.push_back(line);
        }

        file.close();
    }
}


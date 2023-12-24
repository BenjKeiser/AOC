#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

#define DBG 0

void print_map(const std::map<row_t, int>& m)
{
    for (const auto& [key, value] : m)
    {
        std::cout << '[';
        for(int i = 0; i < key.spring_list.size(); i++)
        {
            std::cout << key.spring_list[i];
        } 
        std::cout << ' ';
        for(int i = 0; i < key.damaged_groups.size(); i++)
        {
            std::cout << key.damaged_groups[i] << ",";
        }
        std::cout << "] = " << value << std::endl;;
    }
}

spring_t Elves::symbol_to_spring(char symbol)
{
    spring_t spring;
    if(symbol == '?')
    {
        spring = UNKNOWN;
    }
    else if(symbol == '.')
    {
        spring = OPERATIONAL;
    }
    else if(symbol == '#')
    {
        spring = DAMAGED;
    }
    else
    {
        std::cout << "invalid conversion: " << symbol << std::endl;
        std::exit(-1);
    }

    return spring;
}

struct pos_t {
    int idx;
    spring_t type;
};

uint64_t Elves::get_arrangement(row_t row)
{
    uint64_t arr = 0;

    std::vector<spring_t> springs = row.spring_list;
    std::vector<spring_t> next_springs;
    std::vector<int> damaged = row.damaged_groups;

    int nb_dmg = damaged.front();
    damaged.erase(damaged.begin());

    bool possible = true;

    int left = std::accumulate(damaged.begin(), damaged.end(), 0);
    left += damaged.size();

    if(springs.size() < left)
    {
        // we cannot possibly fit the remainder so we might as well just stop
        return 0;
    }

    auto search = results.find(row);
    if (search == results.end()) 
    {
    #if DBG
        std::cout << "====================" << std::endl;
        std::cout << "Current: " << nb_dmg << std::endl;
        std::cout << "Remaining: " << damaged.size() << std::endl;
        for(auto & v : damaged)
        {
            std::cout << v << ",";
        }
        std::cout << std::endl;

        std::cout << "Springs size: " << springs.size() << std::endl;
    #endif
        for(int i = 0; i < springs.size() - left; i++)
        {
            possible = true;
            if(springs[i] == UNKNOWN || springs[i] == DAMAGED)
            {
                // Try to fit the number of damanged springs
    #if DBG
                std::cout << "Try to fit: " << i << " -> " << i + nb_dmg << std::endl;
    #endif
                for(int k = i; k < i + nb_dmg; k++)
                {
                    if(k >= springs.size())
                    {
                        // We reached the end of the springs before finding a possibility
                        possible = false;
                        break;
                    }
                    if(springs[k] == OPERATIONAL)
                    {
                        // We cannot fit the number of damaged springs
                        possible = false;
                        break;
                    }
                }

                if(possible)
                {
    #if DBG
                    std::cout << "Possible: " << i << " Left: " << damaged.size() << " " << damaged.empty() << std::endl;
    #endif
                    //it is possible and there are no more damaged springs group
                    if(damaged.empty())
                    {
    #if DBG
                        std::cout << "End" << std::endl;
    #endif
                        //check that the remainder of the springs does not contain damaged springs
                        if(std::find(springs.begin()+i+nb_dmg, springs.end(), DAMAGED) != springs.end())
                        {
                            //damaged springs where there should be none
    #if DBG
                            std::cout << "damaged springs where there should be none" << std::endl;
    #endif
                        }
                        else
                        {
                            arr++;
    #if DBG
                            std::cout << "found an arrangement" << std::endl;
    #endif
                        }
                    }
                    else
                    {
                        if(i+nb_dmg < springs.size())
                        {
                            if(springs[i+nb_dmg] != DAMAGED)
                            {
                                next_springs = springs;
                                next_springs.erase(next_springs.begin(), next_springs.begin()+i+nb_dmg+1);
                                arr += get_arrangement({next_springs, damaged});
                            }
                        }
                    }
                }            
            }
            if(springs[i] == DAMAGED)
            {
                //We are at the only possible start for this series
                break;
            }
        }
    #if DBG
        std::cout << "********************" << std::endl;
    #endif

        if(arr > 0)
        {
            results[row] = arr;
#if DBG
            print_map(results);
#endif
        }
    } 
    else 
    {
        //std::cout << "found existing" << std::endl;
        arr = search->second;
    }

    return arr;
}


uint64_t Elves::get_arrangements(int factor)
{
    uint64_t arr = 0;
    row_t row;
    int i = 0;
    for(auto & r : all_springs)
    {
        std::cout << i++ << std::endl;
        row.spring_list = r.spring_list;
        row.damaged_groups = r.damaged_groups;
        for(int i = 0; i < factor - 1; i++)
        {
            row.spring_list.push_back(UNKNOWN);
            row.spring_list.insert(row.spring_list.end(), r.spring_list.begin(), r.spring_list.end());
            row.damaged_groups.insert(row.damaged_groups.end(), r.damaged_groups.begin(), r.damaged_groups.end());
        }
        
#if DBG
        for(auto & s : row.spring_list)
        {
            std::cout << s;
        }
        std::cout << std::endl;

        for(auto & s : row.damaged_groups)
        {
            std::cout << s <<",";
        }
        std::cout << std::endl;
#endif
        results.clear();
        arr += get_arrangement(row);
    }
    return arr;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<spring_t> spring_list;
    std::vector<int> damaged_groups;

    int nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            std::stringstream ss(line);
            while (std::getline(ss, line, ' ')) 
            {
                if(line[0] >= 0x30 && line[0] <= 0x39)  //0-9
                {
                    damaged_groups.clear();
                    std::stringstream ss(line);
                    for (int i; ss >> i;) 
                    {
                        damaged_groups.push_back(i);    
                        if (ss.peek() == ',')
                        {
                            ss.ignore();
                        }
                    }
                    all_springs.push_back({spring_list, damaged_groups});
                }
                else
                {
                    spring_list.clear();
                    for(int i = 0; i < line.length(); i++)
                    {
                        spring_list.push_back(symbol_to_spring(line[i]));
                    }
                }
            }
        }

        file.close();
    }
}


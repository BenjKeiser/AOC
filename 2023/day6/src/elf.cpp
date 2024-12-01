#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

int eval_race(race_t r)
{
    int ways = 0;
    int64_t distance = 0;
    int64_t start = 0;
    int64_t end = 0;
    for(int64_t hold = 0; hold <= r.time; hold++)
    {
        //distance = time left after releasing * time held (1 ms held = 1 dist covered)
        distance = (r.time - hold) * hold;

        //if we beat the record, we win
        if(distance > r.dist)
        {
            start = hold;
            break;
        }
    }

    for(int64_t hold = r.time; hold > 0; hold--)
    {
        //distance = time left after releasing * time held (1 ms held = 1 dist covered)
        distance = (r.time - hold) * hold;

        //if we beat the record, we win
        if(distance > r.dist)
        {
            end = hold;
            break;
        }
    }

    return end-start+1;
}

int Elves::eval_races()
{
    int ways = 1;
    int tmp = 0;
    for(auto & r : races)
    {
        ways *= eval_race(r);
        tmp = 0;
    }

    return ways;
}

int Elves::eval_race_p2()
{
    return eval_race({p2_time, p2_dist});
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<int> time;
    std::vector<int> distance;

    std::string time_str;
    std::string dist_str;

    int pos = 0;
    int nb = 0;

    // parse the file
    if (file.is_open()) 
    {
        time.clear();
        distance.clear();
        while (std::getline(file, line)) 
        {

            //Time Numbers
            pos = line.find("Time:");
            if (std::string::npos != pos)
            {
                std::istringstream t(line);
                std::string temp;
                std::cout << "Times: ";
                while (!t.eof()) 
                {
                    t >> temp;
                    if(std::istringstream(temp) >> nb)
                    {
                        time.push_back(nb);
                        std::cout << std::to_string(nb) << " ";
                    }
                }
                std::cout << std::endl;

                // Get Value for Part 2          
                time_str = line.substr(6);
                time_str.erase(remove(time_str.begin(), time_str.end(), ' '), time_str.end()); //remove ' ' from string
                p2_time = std::stoll(time_str);
                std::cout << "P2 Time: " << std::to_string(p2_time) << std::endl;
            }


            //Distance Numbers
            pos = line.find("Distance:");
            if (std::string::npos != pos)
            {
                std::istringstream d(line);
                std::string temp;
                std::cout << "Distance: ";
                while (!d.eof())
                {
                    d >> temp;
                    if(std::istringstream(temp) >> nb)
                    {
                        distance.push_back(nb);
                        std::cout << std::to_string(nb) << " ";
                    }
                }
                std::cout << std::endl;

                // Get Value for Part 2          
                dist_str = line.substr(9);
                dist_str.erase(remove(dist_str.begin(), dist_str.end(), ' '), dist_str.end()); //remove ' ' from string
                p2_dist = std::stoll(dist_str);
                std::cout << "P2 Dist: " << std::to_string(p2_dist) << std::endl;
            }
        }
        file.close();

        for(int i = 0; i< time.size(); i++)
        {
            races.push_back({time[i], distance[i]});
        }
    }
}


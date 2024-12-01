#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

int get_loc(std::string loc, std::vector<location_t> * locations)
{
    int l = 0;
    int left = 0;
    int right = locations->size() - 1;
    bool found = false;
    std::string mloc;
    while (left <= right)
    {
        int middle = (left + right) / 2;
        mloc = (*locations)[middle].loc;
        //std::cout << loc << " - " << mloc << ": " << std::to_string(left) << ", " << std::to_string(middle) << ", " << std::to_string(right) << std::endl; 
        if(loc == mloc)
        {
            l = middle;
            found = true;
            //std::cout << "found!" << std::endl;
            break;
        }

        for(int i = 0; i < mloc.length(); i++)
        {
            if(mloc[i] != loc[i])
            {
                if(mloc[i] < loc[i])
                {
                    left = middle + 1;
                    break;
                }
                else
                {
                    right = middle - 1;
                    break;
                }
            }
        }
    }
    if(found != true)
    {
        std::cout << "o_O should not get here" << std::endl;
        exit(0);
    }

    return l;
}

int Elves::get_moves()
{
    int moves = 0;
    int iter = 0;
    int loc = 0;

    for(int i = 0; i < steps.length(); i++)
    {
        if(steps[i] == 'L')
        {
            loc = get_loc(locations[loc].left, &locations);
        }
        else
        {
            loc = get_loc(locations[loc].right, &locations);
        }
        if(locations[loc].loc == std::string("ZZZ"))
        {
            moves = i + 1 + iter * steps.length();
            break;
        }
        if(i == steps.length() - 1)
        {
            //we restart, will be increased to 0 on loop begin
            iter++;
            i = -1;
        }
    }

    return moves;
}


int64_t gcd(int64_t a, int64_t b)
{
    for (;;)
    {
        if (a == 0) 
        {
            return b;
        }
        b %= a;
        if (b == 0)
        {
            return a;
        }
        a %= b;
    }
}

int64_t lcm(int64_t a, int64_t b)
{
    int64_t temp = gcd(a, b);

    return temp ? (a / temp * b) : 0;
}

int64_t Elves::get_moves_ghost()
{
    int64_t moves = 0;
    int iter = 0;
    std::vector<int> pos;

    std::vector<int64_t> mov;

    //get starting points
    for(int i = 0; i < locations.size(); i++)
    {
        if(locations[i].loc[2] == 'A')
        {
            pos.push_back(i);
        }
    }

    for(int i = 0; i < steps.length(); i++)
    {
        for(int p = 0; p < pos.size(); p++)
        {
            if(steps[i] == 'L')
            {
                pos[p] = get_loc(locations[pos[p]].left, &locations);
            }
            else
            {
                pos[p] = get_loc(locations[pos[p]].right, &locations);
            }
        }

        std::vector<int>::iterator p;
        for (p = pos.begin(); p != pos.end(); ) 
        {
            if(locations[*p].loc[2] == 'Z')
            {
                moves = i + 1 + iter * steps.length();
                mov.push_back(moves);
                p = pos.erase(p);
            }
            else
            {
                ++p;
            }
        }

        if(pos.empty())
        {
            // we know the moves for all starting positions -> lets find the least common multiple and exit
            moves = std::accumulate(mov.begin(), mov.end(), mov[0], lcm);
            break;
        }

        if(i == steps.length() - 1)
        {
            //we restart, will be increased to 0 on loop begin
            iter++;
            i = -1;
        }
    }

    return moves;
}

bool compare_location(location_t l1, location_t l2)
{
    bool c = false;
    for(int i = 0; i < l1.loc.length(); i++)
    {
        if(l1.loc[i] != l2.loc[i])
        {
            c = l1.loc[i] < l2.loc[i];
            break;
        }
    }
    return c;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    location_t loc;
    // parse the file
    if (file.is_open()) 
    {
        std::getline(file, line);
        steps = line;

        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                loc.loc = line.substr(0, 3);
                loc.left = line.substr(7, 3);
                loc.right = line.substr(12, 3);
                locations.push_back(loc);
            }
        }
        std::sort(locations.begin(), locations.end(), compare_location);

/*
        for(auto & l : locations)
        {
            std::cout << l.loc << std::endl;
        }
*/
        file.close();
    }
}


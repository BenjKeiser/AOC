#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

struct coordinates_t {
    int x;
    int y;
};

void print_galaxy(std::vector<std::string> g)
{
    for(auto & s : g)
    {
        std::cout << s << std::endl;
    }
    std::cout << std::endl;
}


std::vector<std::string> expand_galaxy(std::vector<std::string> * galaxies)
{
    std::vector<std::string> exp_gal = *galaxies;
    std::vector<int> x_i;
    std::vector<int> y_i;

    bool found = false;

    //get x indices to expand
    for(int x = 0; x < exp_gal[0].size(); x++)
    {
        found = false;
        for(int y = 0; y < exp_gal.size(); y++)
        {       
            if(exp_gal[y][x] == '#')
            {
                found = true;
            }
        }
        if(!found)
        {
            x_i.push_back(x);
            //std::cout << "x: " << x << std::endl;
        }
    }

    //get y indices to expand
    for(int y = 0; y < exp_gal.size(); y++)
    {       
        if(std::string::npos == exp_gal[y].find('#'))
        {
            y_i.push_back(y);
            //std::cout << "y: " << y << std::endl;
        }
    }

    //expand x indices
    for(int i = x_i.size()-1; i >= 0; i--)
    {
        for(int y = 0; y < exp_gal.size(); y++)
        {
            exp_gal[y].insert(exp_gal[y].begin()+x_i[i], exp_gal[y][x_i[i]]);
        }
    }

    //expand y indices
    for(int i = y_i.size()-1; i >= 0; i--)
    {
        exp_gal.insert(exp_gal.begin()+y_i[i], exp_gal[y_i[i]]);
    }

    return exp_gal;
}

std::vector<coordinates_t> get_galaxy_coords(std::vector<std::string> * galaxies)
{
    std::vector<coordinates_t> gals;
    for(int y = 0; y < galaxies->size(); y++)
    {
        for(int x = 0; x < (*galaxies)[0].length(); x++)
        {
            if((*galaxies)[y][x] == '#')
            {
                gals.push_back({x,y});
            }
        }
    }

    return gals;
}

int Elves::get_shortest_paths()
{
    int sum = 0;
    std::vector<std::string> e_gal;
    std::vector<coordinates_t> g_coord;
    print_galaxy(galaxy);
    e_gal = expand_galaxy(&galaxy);
    print_galaxy(e_gal);
    g_coord = get_galaxy_coords(&e_gal);

    for(int i = 0; i < g_coord.size(); i++)
    {
        for(int k = i; k < g_coord.size(); k++)
        {
            sum += std::abs(g_coord[i].x - g_coord[k].x) + std::abs(g_coord[i].y - g_coord[k].y);
        }
    }

    return sum;
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
            galaxy.push_back(line);
        }

        file.close();
    }
}


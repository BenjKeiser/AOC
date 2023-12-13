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

struct expansion_t {
    std::vector<int> x_exp;
    std::vector<int> y_exp;
};

void print_galaxy(std::vector<std::string> g)
{
    for(auto & s : g)
    {
        std::cout << s << std::endl;
    }
    std::cout << std::endl;
}


expansion_t expand_galaxy(std::vector<std::string> * galaxies)
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

    return {x_i, y_i};
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


int64_t Elves::get_shortest_paths(int factor)
{
    int64_t sum = 0;
    expansion_t exp_gal;
    std::vector<coordinates_t> g_coord;
    //print_galaxy(galaxy);
    exp_gal = expand_galaxy(&galaxy);
    g_coord = get_galaxy_coords(&galaxy);
    int nb_exp = 0;

    int64_t x_diff = 0;
    int64_t y_diff = 0;

    for(int i = 0; i < g_coord.size(); i++)
    {
        for(int k = i; k < g_coord.size(); k++)
        {
            nb_exp = 0;
            for(auto & x_exp : exp_gal.x_exp)
            {
                if((std::min(g_coord[i].x, g_coord[k].x) < x_exp) && (std::max(g_coord[i].x, g_coord[k].x) > x_exp))
                {
                    nb_exp++;
                }
            }
            x_diff = std::abs(g_coord[i].x - g_coord[k].x) + (nb_exp * (factor - 1));

            nb_exp = 0;
            for(auto & y_exp : exp_gal.y_exp)
            {
                if((std::min(g_coord[i].y, g_coord[k].y) < y_exp) && (std::max(g_coord[i].y, g_coord[k].y) > y_exp))
                {
                    nb_exp++;
                }
            }
            y_diff = std::abs(g_coord[i].y - g_coord[k].y) + (nb_exp * (factor - 1));

            sum += y_diff + x_diff;
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


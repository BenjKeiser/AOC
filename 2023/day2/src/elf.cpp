#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>

#define RED 12
#define GREEN 13
#define BLUE 14

Game::Game(int index)
{
    idx = index;
}

void Game::add_game(game_t game)
{
    //std::cout << "Adding " << std::to_string(game.red) << ", " << std::to_string(game.blue) << ", " << std::to_string(game.green) << std::endl;
    games.push_back(game);
}

int Game::is_possible(game_t balls)
{
    int nb = idx;
    std::cout << "Game: " << std::to_string(nb) << std::endl;
    for(auto & g : games)
    {
        if((balls.red < g.red) || (balls.blue < g.blue) || (balls.green < g.green))
        {
            std::cout << "Not Possible!" << std::endl;
            nb = 0;
        }
    }
    return nb;
}

int Game::get_balls()
{
    int red = 1;
    int blue = 1;
    int green = 1;
    int nb = 0;
    for(auto & g : games)
    {
        if(red < g.red)
        {
            red = g.red;
        }
        if (blue < g.blue)
        {
            blue = g.blue;
        }
        if (green < g.green)
        {
            green = g.green;
        }
    }
    nb = red * blue * green;
    std::cout << std::to_string(nb) << ": "  << std::to_string(red) << ", " << std::to_string(green) << ", "  << std::to_string(blue) <<std::endl;
    return nb;
}


int Elves::get_games()
{
    int sum = 0;
    for(auto & g : games)
    {
        sum += g.is_possible({RED, GREEN, BLUE});
    }
    return sum;
}


int Elves::get_balls()
{
    int sum = 0;
    for(auto & g : games)
    {
        sum += g.get_balls();
    }
    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::string::size_type idx;
    int nb = 0;
    int blue = 0;
    int red = 0;
    int green = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            nb = 0;
            red = 0;
            green = 0;
            blue = 0;
            if(line.length() > 0)
            {
                //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                idx = line.find(" ");
                nb = std::stoi (line.substr(idx + 1));
                //std::cout << "Game: " << std::to_string(nb) << std::endl;
                Game g(nb);

                idx = line.find(":");
                for(int i = idx; i < line.length(); i++)
                {
                    if(line[i] >= 0x30 && line[i] <= 0x39)
                    {
                        nb = std::stoi(line.substr(i), &idx);
                        i = i + idx + 1;
                        if(std::string::npos != line.substr(i, 3).find("red"))
                        {
                            red = nb;
                        }
                        else if(std::string::npos != line.substr(i, 4).find("blue"))
                        {
                            blue = nb;
                        }
                        else if(std::string::npos != line.substr(i, 5).find("green"))
                        {
                            green = nb;
                        }
                        else
                        {
                            std::cout << "Unexpected: " << line.substr(i) << std::endl;
                        }
                    }

                    if(line[i] == ';')
                    {
                        g.add_game({red, green, blue});
                        red = 0;
                        green = 0;
                        blue = 0;
                    }
                }
                g.add_game({red, green, blue});
                games.push_back(g);
            }
        }
        file.close();
    }
}


#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

std::vector<move_t> Elves::get_next_moves(move_t move)
{
    std::vector<move_t> moves;

    if(grid[move.y][move.x] == '.')
    {
        switch(move.dir)
        {
            case UP:
                if(move.y > 0)
                {
                    moves.push_back({move.x, move.y-1, move.dir});
                }
                break;
            case DOWN:
                if(move.y < grid.size()-1)
                {
                    moves.push_back({move.x, move.y+1, move.dir});
                }
                break;
            case LEFT:
                if(move.x > 0)
                {
                    moves.push_back({move.x-1, move.y, move.dir});
                }
                break;
            case RIGHT:
                if(move.x < grid[0].length()-1)
                {
                    moves.push_back({move.x+1, move.y, move.dir});
                }
                break;
            default:
                break;
        }
    }
    else if(grid[move.y][move.x] == '|')
    {
        switch(move.dir)
        {
            case UP:
                if(move.y > 0)
                {
                    moves.push_back({move.x, move.y-1, move.dir});
                }
                break;
            case DOWN:
                if(move.y < grid.size()-1)
                {
                    moves.push_back({move.x, move.y+1, move.dir});
                }
                break;
            case LEFT:
            case RIGHT:
                if(move.y > 0)
                {
                    moves.push_back({move.x, move.y-1, UP});
                }
                if(move.y < grid.size()-1)
                {
                    moves.push_back({move.x, move.y+1, DOWN});
                }
                break;
            default:
                break;
        }
    }
    else if(grid[move.y][move.x] == '-')
    {
        switch(move.dir)
        {
            case UP:
            case DOWN:
                if(move.x > 0)
                {
                    moves.push_back({move.x-1, move.y, LEFT});
                }
                if(move.x < grid[0].length()-1)
                {
                    moves.push_back({move.x+1, move.y, RIGHT});
                }
                break;
            case LEFT:
                if(move.x > 0)
                {
                    moves.push_back({move.x-1, move.y, move.dir});
                }
                break;
            case RIGHT:
                if(move.x < grid[0].length()-1)
                {
                    moves.push_back({move.x+1, move.y, move.dir});
                }
                break;
            default:
                break;
        }
    }
    else if(grid[move.y][move.x] == '/')
    {
        switch(move.dir)
        {
            case UP:
                if(move.x < grid[0].length()-1)
                {
                    moves.push_back({move.x+1, move.y, RIGHT});
                }
                break;
            case DOWN:
                if(move.x > 0)
                {
                    moves.push_back({move.x-1, move.y, LEFT});
                }
                break;
            case LEFT:
                if(move.y < grid.size()-1)
                {
                    moves.push_back({move.x, move.y+1, DOWN});
                }
                break;
            case RIGHT:
                if(move.y > 0)
                {
                    moves.push_back({move.x, move.y-1, UP});
                }
                break;
            default:
                break;
        }
    }
    else if(grid[move.y][move.x] == '\\')
    {
        switch(move.dir)
        {
            case UP:
                if(move.x > 0)
                {
                    moves.push_back({move.x-1, move.y, LEFT});
                }
                break;
            case DOWN:
                if(move.x < grid[0].length()-1)
                {
                    moves.push_back({move.x+1, move.y, RIGHT});
                }
                break;
            case LEFT:
                if(move.y > 0)
                {
                    moves.push_back({move.x, move.y-1, UP});
                }
                break;
            case RIGHT:
                if(move.y < grid.size()-1)
                {
                    moves.push_back({move.x, move.y+1, DOWN});
                }
                break;
            default:
                break;
        }
    }
    else
    {

    }
    return moves;
}

uint64_t Elves::get_energized(move_t start)
{
    uint64_t sum = 0;
    move_t current = start;

    std::vector<move_t> queue;
    std::vector<move_t> moves;

    queue.push_back(current);


    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        //std::cout << current.x << ", " << current.y << " -> " << current.dir << std::endl;

        if(locations[current.y][current.x].visited[current.dir])
        {
            continue;
        }

        locations[current.y][current.x].visited[current.dir] = true;

        moves = get_next_moves(current);
        queue.insert(queue.end(), moves.begin(), moves.end());
    }

    for(int y = 0; y < locations.size(); y++)
    {
        for(int x = 0; x < locations[y].size(); x++)
        {
            if(locations[y][x].is_energized())
            {
                sum++;
            }
        }
    }

    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<location_t> loc;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                grid.push_back(line);

                loc.resize(line.length());
                locations.push_back(loc);
            }
        }
        
        file.close();
    }
}


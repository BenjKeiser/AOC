#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>

#define STRAIGHT_LIMIT 3


enum dir_t {
    WEST = 0,
    SOUTH = 1,
    EAST = 2,
    NORTH = 3,
    START = 4
};

//coordinates struct
struct coordinates_t {
    int x;
    int y;
    dir_t dir;
};

// structure for information of each cell
struct cell {
    int x;
    int y;
    uint64_t distance;
    std::vector<dir_t> dir;
    cell(int x, int y, uint64_t distance, std::vector<dir_t> prev, dir_t step) : x(x), y(y), distance(distance)
    {
        dir = prev;
        if(dir.size() >= 3)
        {
            dir.erase(dir.begin());
        }
        dir.push_back(step);
    }
};

// Utility method for comparing two cells
bool operator<(const cell& a, const cell& b)
{
    if (a.distance == b.distance) 
    {
        if (a.x != b.x)
        {
            return (a.x < b.x);
        }
        else
        {
            return (a.y < b.y);
        }
    }
    return (a.distance < b.distance);
}


//disregarding limits
std::vector<coordinates_t> get_moves(cell k)
{
    std::vector<coordinates_t> moves;

    // direction arrays for simplification of getting neighbour
    int dx[] = { -1, 0, 1, 0 };
    int dy[] = { 0, 1, 0, -1 };

    // looping through all neighbours
    for (int i = 0; i < 4; i++) 
    {
        int delta_x = dx[i];
        int delta_y = dy[i];
        int x = k.x + delta_x;
        int y = k.y + delta_y;


        //we don't move the way back we came
        if(k.dir.size() > 0)
        {
            if((abs(k.dir.back() - i) == 2) && k.dir.back() != START)
            {
                continue;
            }
        }

        int cnt = std::count(k.dir.begin(), k.dir.end(), (dir_t)i);
        if(cnt == STRAIGHT_LIMIT)
        {
            continue;
        }

        moves.push_back({x, y, (dir_t)i});
    }


    return moves;
}

uint64_t Elves::dijkstra()
{
    uint64_t heat_loss = UINT64_MAX;
    int row = heat_map.size();
    int col = heat_map[0].size();

    //set
    std::set<cell> queue;
    std::set<cell> visited;

    //Starting point
    std::vector<dir_t> dir_v;
    queue.insert(cell(0, 0, 0, dir_v, START));

    //dijkstra loop
    while(!queue.empty())
    {
        cell current = *queue.begin();
        queue.erase(queue.begin());

        //check if this was already visited
        std::set<cell>::iterator it = visited.find(current);
        if(it != visited.end())
        {
            continue;
        }

        if(current.x == row - 1 && current.y == col - 1)
        {
            //we reached the end
            heat_loss = current.distance;
            goto end;
        }

        visited.insert(current);

        //get the valid nodes and put them on the queue
        std::vector<coordinates_t> neighbours = get_moves(current);
        for(auto & n : neighbours)
        {
            if(n.y >= 0 && n.y < row && n.x >= 0 && n.x < col)
            {
                //add the node
                queue.insert(cell(n.x, n.y, current.distance + heat_map[n.y][n.x], current.dir, n.dir));
            }
        }
    }

end:
    return heat_loss;
}


uint64_t Elves::get_heat_loss()
{
    uint64_t heat_loss = 0;

    heat_loss = dijkstra();

    return heat_loss;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<int> heat;

    int nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            heat.clear();
            for(int i=0; i < line.length(); i++)
            {
                heat.push_back(line[i]-0x30);
            }
            heat_map.push_back(heat);
        }

        for(int y = 0; y < heat_map.size(); y++)
        {
            for(int x = 0; x < heat_map[0].size(); x++)
            {
                std::cout << heat_map[y][x];
            }
            std::cout << std::endl;
        }
        std::cout << std::endl;

        file.close();
    }
}


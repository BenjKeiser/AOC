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
    cell(int x, int y, uint64_t distance, std::vector<dir_t> prev, dir_t step) : x(x), y(y)
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

    //std::cout << "get_moves [" << k.x << ", " << k.y << " = " << k.distance << "]: " << std::endl;

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
                //std::cout << "Previous: " << x << ", " << y << std::endl;
                continue;
            }
        }

        int cnt = std::count(k.dir.begin(), k.dir.end(), (dir_t)i);
        if(cnt == STRAIGHT_LIMIT)
        {
            //std::cout << "STRAIGHT" << std::endl;
            continue;
        }

        //std::cout << x << ", " << y << std::endl;
        moves.push_back({x, y, (dir_t)i});
    }


    return moves;
}

uint64_t Elves::dijkstra()
{
    int row = heat_map.size();
    int col = heat_map[0].size();
    uint64_t dist[row][col];

    // initializing distance array by INT_MAX
    for (int i = 0; i < row; i++)
    {
        for (int j = 0; j < col; j++)
        {
            dist[i][j] = UINT64_MAX;
        }
    }

    //set
    std::set<cell> st;

    //Starting point
    std::vector<dir_t> dir_v;
    st.insert(cell(0, 0, 0, dir_v, START));

    dist[0][0] = heat_map[0][0];

    //dijkstra loop
    while(!st.empty())
    {
        cell current_min = *st.begin();
        st.erase(st.begin());

        std::vector<coordinates_t> neighbours = get_moves(current_min);
        for(auto & n : neighbours)
        {
            if(n.y >= 0 && n.y < row && n.x >= 0 && n.x < col)
            {
                //check the distance map -> if it is smaller, update
                if(dist[n.y][n.x] >= dist[current_min.y][current_min.x] + heat_map[n.y][n.x])
                {
                    //if there already was a distance / cell, delete it
                    if(dist[n.y][n.x] != UINT64_MAX)
                    {
                        std::set<cell>::iterator it = st.find(cell(n.x, n.y, dist[n.y][n.x], current_min.dir, n.dir));
                        if(it != st.end())
                        {
                            st.erase(it);
                        }
                    }

                    //update
                    dist[n.y][n.x] = dist[current_min.y][current_min.x] + heat_map[n.y][n.x];
                    st.insert(cell(n.x, n.y, dist[n.y][n.x], current_min.dir, n.dir));
                }
            }
        }
    }

    for(int y = 0; y < row; y++)
    {
        for(int x = 0; x < col; x++)
        {
            std::cout << dist[y][x] << " ";
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;

    return dist[row - 1][col - 1];
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


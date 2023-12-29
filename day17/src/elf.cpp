#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>

#define STRAIGHT_LIMIT 3

//coordinates struct
struct coordinates_t {
    int x;
    int y;
};

// structure for information of each cell
struct cell {
    int x;
    int y;
    uint64_t distance;
    cell * prev;
    cell(int x, int y, uint64_t distance, cell * prev) : x(x), y(y), distance(distance), prev(prev)
    {
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

        if(k.prev != NULL)
        {
            //we don't move the way back we came
            if(x == k.prev->x && y == k.prev->y)
            {
                //std::cout << "Previous: " << x << " == " << k.prev->x << ", " << y << " == " << k.prev->y << std::endl;
                continue;
            }

            //check for too long straight line
            cell previous(x,y,0,&k);
            bool straight = true;
            for(int i = 0; i < STRAIGHT_LIMIT; i++)
            {
                if(previous.prev != NULL)
                {
                    if((previous.x != previous.prev->x + delta_x) || (previous.y != previous.prev->y + delta_y))
                    {
                        //std::cout << "Not straight " << i << ": " << previous.x << " != " << previous.prev->x + delta_x << " || " << previous.y << " != " << previous.prev->y + delta_y << std::endl;
                        straight = false;
                        break;
                    }
                    previous = *previous.prev;
                }
                else
                {
                    straight = false;
                }
            }
            if(straight)
            {
                //std::cout << "Straight: " << x << ", " << y << std::endl;
                continue;
            }
        }

        //std::cout << x << ", " << y << std::endl;
        moves.push_back({x, y});
    }


    return moves;
}

uint64_t Elves::dijkstra()
{
    int row = heat_map.size();
    int col = heat_map[0].size();
    uint64_t dist[row][col];
    std::vector<std::vector<cell>> cell_list;

    // initializing distance array by INT_MAX
    for (int i = 0; i < row; i++)
    {
        std::vector<cell> cells;
        cell_list.push_back(cells);
        for (int j = 0; j < col; j++)
        {
            dist[i][j] = UINT64_MAX;
            cell_list[i].push_back(cell(0, 0, 0, NULL));   
        }
    }

    //set
    std::set<cell> st;

    //Starting point
    st.insert(cell(0, 0, 0, NULL));

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
                if(dist[n.y][n.x] > dist[current_min.y][current_min.x] + heat_map[n.y][n.x])
                {
                    //if there already was a distance / cell, delete it
                    if(dist[n.y][n.x] != UINT64_MAX)
                    {
                        st.erase(st.find(cell_list[n.y][n.x]));
                    }

                    //update
                    dist[n.y][n.x] = dist[current_min.y][current_min.x] + heat_map[n.y][n.x];
                    cell update(n.x, n.y, dist[n.y][n.x], &cell_list[current_min.y][current_min.x]);
                    st.insert(update);
                    cell_list[n.y][n.x] = update;
                }
            }
        }
    }

    bool path[row][col];

    for(int y = 0; y < row; y++)
    {
        for(int x = 0; x < col; x++)
        {
            path[y][x] = false;
        }
    }

    cell current = cell_list[row - 1][col - 1];
    path[row - 1][col - 1] = true;
    while(current.prev != NULL)
    {
        path[current.y][current.x] = true;
        current = *current.prev;
    }


    for(int y = 0; y < row; y++)
    {
        for(int x = 0; x < col; x++)
        {
            if(path[y][x])
            {
                std::cout << "x";
            }
            else
            {
                std::cout << heat_map[y][x];
            }
            //std::cout << dist[y][x] << " ";
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


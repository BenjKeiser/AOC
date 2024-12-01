#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>


//Part 1: NOTE: Could also be solved with Algo from Part 2 -> way more efficient
struct pos_t {
    bool dug;
    bool visited;
    std::string colour;
};

direction_t char_to_dir(char dir)
{
    if(dir == 'U')
    {
        return U;
    }
    if(dir == 'D')
    {
        return D;
    }
    if(dir == 'R')
    {
        return R;
    }
    if(dir == 'L')
    {
        return L;
    }
    return F;
}

void print_lagoon(std::vector<std::vector<pos_t>> * lag)
{
    std::vector<std::vector<pos_t>> lagoon = *lag;
    for(int y = 0; y < lagoon.size(); y++)
    {
        for(int x = 0; x < lagoon[y].size(); x++)
        {
            if(lagoon[y][x].dug || !lagoon[y][x].visited)
            {
                std::cout << '#';
            }
            else
            {
                std::cout << '.';
            }
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}

std::vector<std::vector<pos_t>> build_lagoon(std::pair<int, int> limits)
{
    std::vector<std::vector<pos_t>> lagoon;
    std::vector<pos_t> row;
    pos_t state = {false, false, ""};

    for(int x = 0; x < limits.first; x++)
    {
        row.push_back(state);
    }

    for(int y = 0; y < limits.second; y++)
    {
        lagoon.push_back(row);

    }

    return lagoon;
}

std::pair<std::pair<int,int>, std::pair<int, int>> get_limits(std::vector<dig_t> * dig_orders)
{
    std::pair<int, int> limits = {0, 0};
    std::pair<int, int> limits_row = {0, 0}; // y
    std::pair<int, int> limits_col = {0, 0}; // x
    std::pair<int, int> position = {0, 0};

    for(auto & dig : *dig_orders)
    {
        if(dig.direction == U)
        {
            position.second -= dig.length;
        }
        else if(dig.direction == R)
        {
            position.first += dig.length;
        }
        else if(dig.direction == D)
        {
            position.second += dig.length;
        }
        else if(dig.direction == L)
        {
            position.first -= dig.length;
        }
        else
        {
            std::cout << "Shouldn't get here!" << std::endl;
            std:exit(-1);
        }

        if(position.first < limits_col.first)
        {
            limits_col.first = position.first;
        }
        if(position.first > limits_col.second)
        {
            limits_col.second = position.first;
        }
        if(position.second < limits_row.first)
        {
            limits_row.first = position.second;
        }
        if(position.second > limits_row.second)
        {
            limits_row.second = position.second;
        }
    }

    limits.first = abs(limits_col.first) + abs(limits_col.second) + 1;
    limits.second = abs(limits_row.first) + abs(limits_row.second) + 1;

    position.first = abs(limits_col.first);
    position.second = abs(limits_row.first);

    //std::cout << "ROWS: " << limits_row.first << " -> " << limits_row.second << std::endl;
    //std::cout << "COLS: " << limits_col.first << " -> " << limits_col.second << std::endl;
    //std::cout << "LIMITS: " << limits.first << " -> " << limits.second << std::endl;
    //std::cout << "START: " << position.first << " -> " << position.second << std::endl;

    return {limits, position};
}

std::vector<std::vector<pos_t>> dig_trench(std::vector<dig_t> * dig_orders, std::vector<std::vector<pos_t>> * lag, std::pair<int, int> start)
{
    int x = start.first;
    int y = start.second;
    std::vector<std::vector<pos_t>> lagoon = *lag;
    int dx = 0;
    int dy = 0;
    lagoon[y][x].dug = true;
    for(auto & dig : *dig_orders)
    {
        if(dig.direction == U)
        {
            dx = 0;
            dy = -1;
        }
        else if(dig.direction == R)
        {
            dx = 1;
            dy = 0;
        }
        else if(dig.direction == D)
        {
            dx = 0;
            dy = 1;
        }
        else if(dig.direction == L)
        {
            dx = -1;
            dy = 0;
        }
        else
        {
            std::cout << "Shouldn't get here!" << std::endl;
            std:exit(-1);
        }
        for(int i = 0; i < dig.length; i++)
        {
            y += dy;
            x += dx;
            if(x >= 0 && x < lagoon[0].size() && y >= 0 && y < lagoon.size())
            {
                lagoon[y][x].dug = true;
                lagoon[y][x].colour = dig.colour;
            }
            else
            {
                std::cout << "o_O: " << x << ", " << y << std::endl;
                std::exit(-1);
            }
        }
    }
    return lagoon;
}

std::vector<std::pair<int, int>> get_neighbours(int x, int y, int row, int col)
{
    std::vector<std::pair<int, int>> neighbours;

    int dx[] = { -1, 0, 1, 0 };
    int dy[] = { 0, 1, 0, -1 };

    // looping through all neighbours
    for (int i = 0; i < 4; i++) 
    {
        int nx = x + dx[i];
        int ny = y + dy[i];

        if(nx >= 0 && nx < col && ny >= 0 && ny < row)
        {
            neighbours.push_back({nx, ny});
        }
    }   

    return neighbours;
}

std::vector<std::vector<pos_t>> dig_between(std::vector<std::vector<pos_t>> * lag)
{
    std::vector<std::vector<pos_t>> lagoon = *lag;
    std::vector<std::pair<int,int>> queue;

    //get all edge nodes
    for(int y = 0; y < lagoon.size(); y++)
    {
        for(int x = 0; x < lagoon[0].size(); x++)
        {
            if(x == 0 || x == lagoon[0].size()-1 || y == 0 || y == lagoon.size()-1)
            {
                if(!lagoon[y][x].dug)
                {
                    //not part of the trench, we will start exploring here
                    queue.push_back({x, y});
                }
            }
        }
    }

    std::pair<int,int> current;
    int x = 0;
    int y = 0;

    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        x = current.first;
        y = current.second;

        if(lagoon[y][x].visited)
        {
            //we were already here;
            continue;
        }

        lagoon[y][x].visited = true;

        //get the neighbours
        std::vector<std::pair<int, int>> neighbours = get_neighbours(x, y, lagoon.size() - 1, lagoon[0].size() - 1);
        for(auto & n : neighbours)
        {
            if(!lagoon[n.second][n.first].dug)
            {
                //it is not a trench -> we can explore
                queue.push_back(n);
            }
        }
    }

    return lagoon;
}

uint64_t get_holes(std::vector<std::vector<pos_t>> * lag)
{
    uint64_t holes = 0;
    std::vector<std::vector<pos_t>> lagoon = *lag;
    for(int y = 0; y < lagoon.size(); y++)
    {
        for(int x = 0; x < lagoon[y].size(); x++)
        {
            if(lagoon[y][x].dug || !lagoon[y][x].visited)
            {
                holes++;
            }
        }
    }

    return holes;
}

uint64_t Elves::get_lagoon()
{
    uint64_t hole = 0;
    auto [limits, start] = get_limits(&dig_orders);
    std::vector<std::vector<pos_t>> lagoon = build_lagoon(limits);
    lagoon = dig_trench(&dig_orders, &lagoon, start);
    //print_lagoon(&lagoon);
    lagoon = dig_between(&lagoon);
    //print_lagoon(&lagoon);
    hole = get_holes(&lagoon);

    return hole;
}

// Part 2 -> Area of Polygon
/*  https://en.wikipedia.org/wiki/Polygon#Area 
    https://en.wikipedia.org/wiki/Shoelace_formula 
    https://en.wikipedia.org/wiki/Pick%27s_theorem */

std::vector<dig_t> translate_dig(std::vector<dig_t> * dig_orders)
{
    std::vector<dig_t> orders;
    dig_t dig;
    for(auto & d : *dig_orders)
    {
        dig.direction = (direction_t)(d.colour[6] - '0');
        dig.length = std::stoi(d.colour.substr(1, 5), NULL, 16);
        dig.colour = "";
        orders.push_back(dig);
    }
    return orders;
}

uint64_t get_corners(std::vector<dig_t> * dig_orders, std::vector<std::pair<int64_t, int64_t>> * corners)
{
    std::pair<int64_t, int64_t> limits = {0, 0};
    std::pair<int64_t, int64_t> position = {0, 0};
    uint64_t border_points = 0;

    for(auto & dig : *dig_orders)
    {
        if(dig.direction == U)
        {
            position.second -= dig.length;
        }
        else if(dig.direction == R)
        {
            position.first += dig.length;
        }
        else if(dig.direction == D)
        {
            position.second += dig.length;
        }
        else if(dig.direction == L)
        {
            position.first -= dig.length;
        }
        else
        {
            std::cout << "Shouldn't get here!" << std::endl;
            std:exit(-1);
        }

        border_points += dig.length;
        corners->push_back(position);
        //std::cout << position.first << ", " << position.second << std::endl;

        if(position.first < limits.first)
        {
            limits.first = position.first;
        }
        if(position.second < limits.second)
        {
            limits.second = position.second;
        }
    }

    for(auto & p: *corners)
    {
        p.first += abs(limits.first);
        p.second += abs(limits.second);
        //std::cout << p.first << ", " << p.second << std::endl;
    }

    return border_points;
}

int64_t get_area(std::vector<std::pair<int64_t, int64_t>> * c)
{
    int64_t area = 0;
    int64_t a = 0;
    std::vector<std::pair<int64_t, int64_t>> corners = *c;
    std::pair<int64_t, int64_t> x0;
    std::pair<int64_t, int64_t> x1;

    x0 = corners.back();
    corners.pop_back();
    corners.insert(corners.begin(), x0);

    while(!corners.empty())
    {
        x1 = corners.back();
        corners.pop_back();
        a = ((x0.first - x1.first) * (x0.second + x1.second)) / 2;
        area += a;
        //std::cout << "A " << a << "[" << area << "] = (" << x0.first << " - " << x1.first << ") * (" << x0.second << " + " << x1.second << ") / 2" << std::endl;
        x0 = x1;
    }
    return llabs(area);
}

uint64_t Elves::get_lagoon_colour()
{
    uint64_t hole = 0;
    std::vector<dig_t> colour_orders = translate_dig(&dig_orders);
    std::vector<std::pair<int64_t, int64_t>> corners;
    uint64_t border_points = get_corners(&colour_orders, &corners);

    //std::cout << "BP: " << border_points << std::endl;

    hole = get_area(&corners);

    //with picks we can calculate the sum of interior + border points which is what we want in this exercise
    hole = hole + border_points / 2 + 1;

    return hole;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    dig_t dig_order;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                dig_order.direction= char_to_dir(line[0]);
                dig_order.length=std::stoi(line.substr(2));
                int pos = line.find('#');
                dig_order.colour = line.substr(pos, 7);

                dig_orders.push_back(dig_order);
            }
        }
        
        file.close();
    }
}


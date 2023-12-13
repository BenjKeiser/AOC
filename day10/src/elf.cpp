#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

void Elves::print()
{
    for(int y = 0; y < pipes.size(); y++)
    {
        for(int x = 0; x < pipes[y].length(); x++)
        {
            if(locations[y][x].visited)
            {
                //std::cout << locations[y][x].steps;
                std::cout << "o";
            }
            else
            {
                //std::cout << pipes[y][x];
                std::cout << ".";
            }
        }
        std::cout << std::endl;
    }
}

void Elves::print_nest()
{
    for(int y = 0; y < pipes.size(); y++)
    {
        for(int x = 0; x < pipes[y].length(); x++)
        {
            if(locations[y][x].visited)
            {
                std::cout << pipes[y][x];
            }
            else
            {
                if(locations[y][x].nest)
                {
                    std::cout << "I";
                }
                else
                {
                    std::cout << "O";
                }
            }
        }
        std::cout << std::endl;
    }
}

std::vector<coordinates_t> Elves::get_next(int x, int y)
{
    std::vector<coordinates_t> moves;
    char symbol = pipes[y][x];

    if(symbol == 'S')
    {
        // NORTH
        if((y > 0) && ((pipes[y-1][x] == '|') || (pipes[y-1][x] == '7') || (pipes[y-1][x] == 'F')))
        {
            moves.push_back({x, y-1});
        }
        
        // WEST
        if((x > 0) && ((pipes[y][x-1] == '-') || (pipes[y][x-1] == 'L') || (pipes[y][x-1] == 'F')))
        {
            moves.push_back({x-1, y});
        }
        
        // SOUTH
        if((y < pipes.size()-1) && ((pipes[y+1][x] == '|') || (pipes[y+1][x] == 'L') || (pipes[y+1][x] == 'J')))
        {
            moves.push_back({x, y+1});
        }
        
        // EAST
        if((x < pipes[y].length()-1) && ((pipes[y][x+1] == '-') || (pipes[y][x+1] == 'J') || (pipes[y][x+1] == '7')))
        {
            moves.push_back({x+1, y});
        }
    }
    else if(symbol == '|')
    {
        // NORTH
        if((y > 0) && ((pipes[y-1][x] == '|') || (pipes[y-1][x] == '7') || (pipes[y-1][x] == 'F')))
        {
            moves.push_back({x, y-1});
        }
        
        // SOUTH
        if((y < pipes.size()-1) && ((pipes[y+1][x] == '|') || (pipes[y+1][x] == 'L') || (pipes[y+1][x] == 'J')))
        {
            moves.push_back({x, y+1});
        }
    }
    else if(symbol == '-')
    {
        // WEST
        if((x > 0) && ((pipes[y][x-1] == '-') || (pipes[y][x-1] == 'L') || (pipes[y][x-1] == 'F')))
        {
            moves.push_back({x-1, y});
        }
        
        // EAST
        if((x < pipes[y].length()-1)  && ((pipes[y][x+1] == '-') || (pipes[y][x+1] == 'J') || (pipes[y][x+1] == '7')))
        {
            moves.push_back({x+1, y});
        }
    }
    else if(symbol == 'L')
    {
        // NORTH
        if((y > 0) && ((pipes[y-1][x] == '|') || (pipes[y-1][x] == '7') || (pipes[y-1][x] == 'F')))
        {
            moves.push_back({x, y-1});
        }

        // EAST
        if((x < pipes[y].length()-1) && ((pipes[y][x+1] == '-') || (pipes[y][x+1] == 'J') || (pipes[y][x+1] == '7')))
        {
            moves.push_back({x+1, y});
        }
    }
    else if(symbol == 'J')
    {
        // NORTH
        if((y > 0) && ((pipes[y-1][x] == '|') || (pipes[y-1][x] == '7') || (pipes[y-1][x] == 'F')))
        {
            moves.push_back({x, y-1});
        }

        // WEST
        if((x > 0) && ((pipes[y][x-1] == '-') || (pipes[y][x-1] == 'L') || (pipes[y][x-1] == 'F')))
        {
            moves.push_back({x-1, y});
        }
    }
    else if(symbol == '7')
    {
        // WEST
        if((x > 0) && ((pipes[y][x-1] == '-') || (pipes[y][x-1] == 'L') || (pipes[y][x-1] == 'F')))
        {
            moves.push_back({x-1, y});
        }

        // SOUTH
        if((y < pipes.size()-1) && ((pipes[y+1][x] == '|') || (pipes[y+1][x] == 'L') || (pipes[y+1][x] == 'J')))
        {
            moves.push_back({x, y+1});
        }
    }
    else if(symbol == 'F')
    {
        // EAST
        if((x < pipes[y].length()-1) && ((pipes[y][x+1] == '-') || (pipes[y][x+1] == 'J') || (pipes[y][x+1] == '7')))
        {
            moves.push_back({x+1, y});
        }

        // SOUTH
        if((y < pipes.size()-1) && ((pipes[y+1][x] == '|') || (pipes[y+1][x] == 'L') || (pipes[y+1][x] == 'J')))
        {
            moves.push_back({x, y+1});
        }
    }
    else
    {
        // no symbol with possible moves
    }

    return moves;
}


int Elves::get_farthest()
{
    int steps = 0;

    std::vector<coordinates_t> queue;

    queue.push_back(start);
    
    coordinates_t current;

    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        if(locations[current.y][current.x].visited)
        {
            // we already where at this location, no need to visit it again
            continue;
        }

        //lets visit the location
        locations[current.y][current.x].visited = true;

        //check the neighbours
        locations[current.y][current.x].next = get_next(current.x, current.y);

        for(auto & l : locations[current.y][current.x].next)
        {
            if(!locations[l.y][l.x].visited)
            {
                //if a location was not visited, assign the proper steps
                locations[l.y][l.x].steps = locations[current.y][current.x].steps+1;
                queue.push_back(l);
            }
            else
            {
                //location was already visited, lets see if it is the loop we're looking for
                if(steps < locations[l.y][l.x].steps)
                {
                    steps = locations[l.y][l.x].steps;
                }

                if(steps < locations[current.y][current.x].steps)
                {
                    steps = locations[current.y][current.x].steps;
                }
            }
        }
    }

    return steps;
}

// todo: checks for diagonal moves?
// Todo: Handle S
std::vector<coordinates_t> Elves::get_outside_nodes(wall_t pos)
{
    int x = pos.coord.x;
    int y = pos.coord.y;
    direction_t wall = pos.wall;
    direction_t dir = pos.dir;

    std::cout << "Outside Node: " << x << ", " << y << std::endl;
    std::vector<coordinates_t> coord;

    char symbol = pipes[y][x];
    switch(dir)
    {
        case UP:
            if(symbol == '|')
            {
                //check if a gap opened
                if((wall == LEFT) && (x > 0) && !locations[y][x-1].visited)
                {
                    coord.push_back({x-1, y});
                }
                else if((wall == RIGHT) && (x < pipes[y].length()-1) && !locations[y][x+1].visited)
                {
                    coord.push_back({x+1, y});
                }
            }
            else if(symbol == 'L')
            {
                if((wall == LEFT) && (x>0) && !locations[y][x-1].visited)
                {
                    coord.push_back({x-1, y});
                }
            }
            else if(symbol == 'J')
            {
                if((wall == RIGHT) && (x<pipes[y].length()) && !locations[y][x+1].visited)
                {
                    coord.push_back({x+1, y});
                }
            }
            else if(symbol == 'F')
            {
                //we reached a bend, check where the wall is, otherwise it might be a dead end
                if((wall == LEFT) && (x > 0) && !locations[y][x-1].visited)
                {
                    //gap opened up
                    coord.push_back({x-1, y});
                }
                
                if((wall == LEFT) && (y > 0) && !locations[y-1][x].visited)                
                {
                    //gap ahead
                    coord.push_back({x, y-1});
                }
            }
            else if(symbol == '7')
            {
               if((wall == RIGHT) && (x < pipes[y].length() - 1) && !locations[y][x+1].visited)
                {
                    //gap opened up
                    coord.push_back({x+1, y});
                }
                
                if(((wall == RIGHT) && y > 0) && !locations[y-1][x].visited)                
                {
                    //gap ahead
                    coord.push_back({x, y-1});
                }
            }
            break;
        case DOWN:
            if(symbol == '|')
            {
                //check if a gap opened
                if((wall == RIGHT) && (x > 0) && !locations[y][x-1].visited)
                {
                    coord.push_back({x-1, y});
                }
                else if((wall == LEFT) && (x < pipes[y].length()-1) && !locations[y][x+1].visited)
                {
                    coord.push_back({x+1, y});
                }
            }
            else if(symbol == 'F')
            {
                if((wall == RIGHT) && (x>0) && !locations[y][x-1].visited)
                {
                    coord.push_back({x-1, y});
                }
            } 
            else if(symbol == '7')
            {
                if((wall == LEFT) && (x<pipes[y].length()-1) && !locations[y][x+1].visited)
                {
                    coord.push_back({x+1, y});
                }
            }       
            else if(symbol == 'L')
            {
                //we reached a bend, check where the wall is, otherwise it might be a dead end
                if((wall == RIGHT) && (x > 0) && !locations[y][x-1].visited)
                {
                    //gap opened up
                    coord.push_back({x-1, y});
                }

                if((wall == RIGHT) && (y < pipes.size()-1) && !locations[y+1][x].visited)                
                {
                    //gap ahead
                    coord.push_back({x, y+1});
                }
            }
            else if(symbol == 'J')
            {
               if((wall == LEFT) && (x < pipes[y].length()-1) && !locations[y][x+1].visited)
                {
                    //gap opened up
                    coord.push_back({x+1, y});
                }
                
                if((wall == LEFT) && (y > 0) && !locations[y+1][x].visited)                
                {
                    //gap ahead
                    coord.push_back({x, y+1});
                }
            }
            break;
        case LEFT:
            if(symbol == '-')
            {
                //check if a gap opened
                if((wall == UP) && (y > 0) && !locations[y-1][x].visited)
                {
                    coord.push_back({x, y-1});
                }
                else if((wall == DOWN) && (y < pipes.size()-1) && !locations[y+1][x].visited)
                {
                    coord.push_back({x, y+1});
                }
            }
            else if(symbol == '7')
            {
                if((wall == UP) && (y>0) && !locations[y-1][x].visited)
                {
                    coord.push_back({x, y-1});
                }
            }
            else if(symbol == 'J')
            {
                if((wall == DOWN) && (y<pipes.size()-1) && !locations[y+1][x].visited)
                {
                    coord.push_back({x, y+1});
                }
            }
            else if(symbol == 'F')
            {
                if((wall == UP) && (y > 0) && !locations[y-1][x].visited)
                {
                    //gap opened up
                    coord.push_back({x, y-1});
                }
                
                if((wall == UP) && (x > 0) && !locations[y][x-1].visited)                
                {
                    //gap ahead
                    coord.push_back({x-1, y});
                }
            }
            else if(symbol == 'L')
            {
                if((wall == DOWN) && (y < pipes.size()-1) && !locations[y+1][x].visited)
                {
                    //gap opened up
                    coord.push_back({x, y+1});
                }

                if((wall == DOWN) && (x > 0) && !locations[y][x-1].visited)                
                {
                    //gap ahead
                    coord.push_back({x-1, y});
                }
            }
            break;
        case RIGHT:
            if(symbol == '-')
            {
                //check if a gap opened
                if((wall == UP) && (y > 0) && !locations[y-1][x].visited)
                {
                    coord.push_back({x, y-1});
                }
                else if((wall == DOWN) && (y < pipes.size()-1) && !locations[y+1][x].visited)
                {
                    coord.push_back({x, y+1});
                }
            }
            else if(symbol == 'F')
            {
                if((wall == UP) && (y > 0) && !locations[y-1][x].visited)
                {
                    coord.push_back({x, y-1});
                }
            }
            else if(symbol == 'L')
            {
                if((wall == DOWN) && (y < pipes[y].length()-1) && !locations[y+1][x].visited)
                {
                    coord.push_back({x, y+1});
                }
            }
            else if(symbol == '7')
            {
                if((wall == UP) && (y > 0) && !locations[y-1][x].visited)
                {
                    //gap opened up
                    coord.push_back({x, y-1});
                }

                if((wall == UP) && (x < pipes[y].length()-1) && !locations[y][x+1].visited)                
                {
                    //gap ahead
                    coord.push_back({x+1, y});
                }
            }
            else if(symbol == 'J')
            {
                if((wall == DOWN) && (y < pipes.size()) && !locations[y+1][x].visited)
                {
                    //gap opened up
                    coord.push_back({x, y+1});
                }
                
                if((wall == DOWN) && (x < pipes[y].length()-1) && !locations[y][x+1].visited)                
                {
                    //gap ahead
                    coord.push_back({x+1, y});
                }
            }
            break;
    }
    return coord;
}

wall_t Elves::get_next_wall(wall_t pos)
{
    int x = pos.coord.x;
    int y = pos.coord.y;
    direction_t wall = pos.wall;
    direction_t dir = pos.dir;

    wall_t next_wall;

    char symbol = pipes[y][x];

    std::cout << "Wall: " <<  symbol << " -> " << x << ", " << y << "; " << wall << ", " << dir << std::endl;

    switch(dir)
    {
        case UP:
            if((symbol == '|') || (symbol == 'L') || (symbol == 'J'))
            {
                next_wall = {{x, y-1}, wall, dir};
            }
            else if(symbol == 'F')
            {
                if(wall == RIGHT)
                {
                    next_wall = {{x+1, y}, DOWN, RIGHT};
                }
                else if(wall == LEFT)
                {
                    next_wall = {{x+1, y}, UP, RIGHT};
                }
            }
            else if(symbol == '7')
            {
                if(wall == RIGHT)
                {
                    next_wall = {{x-1, y}, UP, LEFT};
                }
                else if(wall == LEFT)
                {
                    next_wall = {{x-1, y}, DOWN, LEFT};
                }
            }
            break;
        case DOWN:
            if((symbol == '|') || (symbol == 'F') || (symbol == '7'))
            {
                next_wall = {{x, y+1}, wall, dir};
            } 
            else if(symbol == 'L')
            {
                if(wall == LEFT)
                {
                    next_wall = {{x+1, y}, UP, LEFT};
                }
                else if(wall == RIGHT)
                {
                    next_wall = {{x+1, y}, DOWN, LEFT};
                }
            }
            else if(symbol == 'J')
            {
                if(wall == LEFT)
                {
                    next_wall = {{x-1, y}, DOWN, RIGHT};
                }
                else if(wall == RIGHT)
                {
                    next_wall = {{x-1, y}, UP, RIGHT};
                }
            }
            break;
        case LEFT:
            if((symbol == '-') || (symbol == '7') || (symbol == 'J'))
            {                
                next_wall = {{x-1, y}, wall, dir};
            }
            else if(symbol == 'F')
            {
                if(wall == DOWN)
                {
                    next_wall = {{x, y+1}, LEFT, DOWN};
                }
                else if(wall == UP)
                {
                    next_wall = {{x, y+1}, RIGHT, DOWN};
                }
            }
            else if(symbol == 'L')
            {
                if(wall == DOWN)
                {
                    next_wall = {{x, y-1}, LEFT, UP};
                }
                else if(wall == UP)
                {
                    next_wall = {{x, y-1}, RIGHT, UP};
                }
            }
            break;
        case RIGHT:
            if((symbol == '-') || (symbol == 'F') || (symbol == 'L'))
            {
                next_wall = {{x+1, y}, wall, dir};
            }
            else if(symbol == '7')
            {
                if(wall == DOWN)
                {
                    next_wall = {{x, y+1}, RIGHT, DOWN};
                }
                else if(wall == UP)
                {
                    next_wall = {{x, y+1}, LEFT, DOWN};
                }
            }
            else if(symbol == 'J')
            {
                if(wall == DOWN)
                {
                    next_wall = {{x, y-1}, RIGHT, UP};
                }
                else if(wall == UP)
                {
                    next_wall = {{x, y-1}, LEFT, UP};
                }
            }
            break;
    }

    //std::cout << "Next: " << next_wall.coord.x << ", " << next_wall.coord.y << std::endl;
    return next_wall;
}

std::vector<coordinates_t> Elves::explore_tube(int x, int y, direction_t wall, direction_t dir)
{
    // we explore a tube until we find an opening
    std::vector<wall_t> queue;

    std::cout << "explore_tube: " << x << ", " << y << std::endl;

    queue.push_back({{x, y}, wall, dir});
    
    std::vector<coordinates_t> coord;
    std::vector<coordinates_t> outside;
    wall_t current;
    wall_t next;

    while(!queue.empty())
    {
        current = queue.back();
        queue.pop_back();

        if(locations[current.coord.y][current.coord.x].nest_visited)
        {
            // we already where at this location, no need to visit it again
            continue;
        }

        //lets visit the location
        locations[current.coord.y][current.coord.x].nest_visited = true;

        //check if there is an inside node
        outside = get_outside_nodes(current);
        coord.insert(coord.end(), outside.begin(), outside.end());

        //get the next moves along the wall
        next = get_next_wall(current);
        queue.push_back(next);
    }

    std::cout << "explore_tube done" << std::endl;
    return coord;
}

void Elves::replace_S()
{
    int x = start.x;
    int y = start.y;
    int x_1 = locations[start.y][start.x].next[0].x;
    int y_1 = locations[start.y][start.x].next[0].y;
    int x_2 = locations[start.y][start.x].next[1].x;
    int y_2 = locations[start.y][start.x].next[1].y;
    int x_diff = x + x_1 + x_2 - 3*x;
    int y_diff = y + y_1 + y_2 - 3*y;;


    if(y_1 == y_2)
    {
        pipes[y][x] = '|';
    }
    else if(x_1 == x_2)
    {
        pipes[y][x] = '-';
    }
    else if((x_diff < 0) && ( y_diff < 0 ))
    {
        pipes[y][x] = 'J';
    }
    else if((x_diff > 0) && ( y_diff < 0 ))
    {
        pipes[y][x] = 'L';
    }
    else if((x_diff < 0) && ( y_diff > 0 ))
    {
        pipes[y][x] = '7';
    }
    else if((x_diff > 0) && ( y_diff > 0 ))
    {
        pipes[y][x] = 'F';
    }
    else
    {
        std::cout << "SHOULD NOT GET HERE" << std::endl;
        std::exit(0);
    }
}

std::vector<coordinates_t> Elves::get_next_nest(int x, int y)
{
    std::vector<coordinates_t> possible_nodes;
    std::vector<coordinates_t> from_tube;
    bool is_wall = locations[y][x].visited;
    char symbol = '.';
    if(is_wall)
    {
        //we have entered a tube from the west
        if(x == 0)
        {
            if(symbol == 'F')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((y > 0) && locations[y-1][x].visited)
                {
                    from_tube = explore_tube(x+1, y, UP, RIGHT);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
            if(symbol == 'L')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((y < pipes.size()-1) && locations[y+1][x].visited)
                {
                    from_tube = explore_tube(x+1, y, DOWN, RIGHT);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
        }

        //we have entered a tube from the east
        if(x == pipes[y].length()-1)
        {
            if(symbol == '7')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((y > 0) && locations[y-1][x].visited)
                {
                    from_tube = explore_tube(x-1, y, UP, LEFT);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
            if(symbol == 'J')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((y < pipes.size()-1) && locations[y+1][x].visited)
                {
                    from_tube = explore_tube(x-1, y, DOWN, LEFT);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
        }

        //we have entered a tube from the north
        if(y == 0)
        {
            if(symbol == '7')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((x < pipes[y].length()-1) && locations[y][x+1].visited)
                {
                    from_tube = explore_tube(x, y+1, LEFT, DOWN);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
            if(symbol == 'F')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((x > 0) && locations[y][x-1].visited)
                {
                    from_tube = explore_tube(x, y+1, RIGHT, DOWN);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
        }

        //we have entered a tube from the south
        if(y == pipes.size()-1)
        {
            if(symbol == 'J')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((x < pipes[y].length()-1) && locations[y][x+1].visited)
                {
                    from_tube = explore_tube(x, y-1, RIGHT, UP);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
            if(symbol == 'L')
            {
                //check for wall, otherwise there is another possibility to get there without exploring the tube
                if((x > 0) && locations[y][x-1].visited)
                {
                    from_tube = explore_tube(x, y-1, LEFT, UP);
                    possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                }
            }
        }
    }
    else
    {
        //we might be a nest node and have to check in all directions
        //north west
        if((x > 0 && y > 0))
        {
            if(!locations[y-1][x-1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x-1, y-1});
            }
            else
            {
                //we don't evaluate diagonal walls
            }
        }

        //north
        if(y > 0)
        {
            if(!locations[y-1][x].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x, y-1});
            }
            else
            {
                // part of the wall, we need to evaluate if we can move alongside it
                symbol = pipes[y-1][x];
                if(symbol == 'J')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((x < pipes[y].length()-1) && locations[y-1][x+1].visited)
                    {
                        from_tube = explore_tube(x, y-1, RIGHT, UP);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
                if(symbol == 'L')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((x > 0) && locations[y-1][x-1].visited)
                    {
                        from_tube = explore_tube(x, y-1, LEFT, UP);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
            }
        }

        //north east
        if((x < pipes[y].length()-1 && y > 0))
        {
            if(!locations[y-1][x+1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x+1, y-1});
            }
            else
            {
                //we don't evaluate diagonal walls
            }
        }

        //west
        if(x > 0)
        {
            if(!locations[y][x-1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x-1, y});
            }
            else
            {
                // part of the wall, we need to evaluate if we can move alongside it
                symbol = pipes[y][x-1];
                if(symbol == 'J')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((y < pipes.size()-1) && locations[y+1][x-1].visited)
                    {
                        from_tube = explore_tube(x-1, y, DOWN, LEFT);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
                if(symbol == '7')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((y > 0) && locations[y-1][x-1].visited)
                    {
                        from_tube = explore_tube(x-1, y, UP, LEFT);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
            }
        }

        //east
        if(x < pipes[y].length()-1)
        {
            if(!locations[y][x+1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x+1, y});
            }
            else
            {
                // part of the wall, we need to evaluate if we can move alongside it
                symbol = pipes[y][x+1];
                if(symbol == 'L')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((y < pipes.size()-1) && locations[y+1][x+1].visited)
                    {
                        from_tube = explore_tube(x+1, y, DOWN, RIGHT);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
                if(symbol == 'F')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((y > 0) && locations[y-1][x+1].visited)
                    {
                        from_tube = explore_tube(x+1, y, UP, RIGHT);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
            }
        }

        //south west
        if((x > 0 && y < pipes.size()-1))
        {
            if(!locations[y+1][x-1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x-1, y+1});
            }
            else
            {
                //we don't evaluate diagonal walls
            }
        }

        //south
        if(y < pipes.size() - 1)
        {
            if(!locations[y+1][x].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x, y+1});
            }
            else
            {
                // part of the wall, we need to evaluate if we can move alongside it
                symbol = pipes[y+1][x];
                if(symbol == '7')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((x < pipes[y].length()-1) && locations[y+1][x+1].visited)
                    {
                        from_tube = explore_tube(x, y+1, LEFT, DOWN);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
                if(symbol == 'F')
                {
                    //check for wall, otherwise there is another possibility to get there without exploring the tube
                    if((x > 0) && locations[y+1][x-1].visited)
                    {
                        from_tube = explore_tube(x, y+1, RIGHT, DOWN);
                        possible_nodes.insert(possible_nodes.end(), from_tube.begin(), from_tube.end());
                    }
                }
            }
        }

        //south east
        if((x < pipes[y].length()-1 && y < pipes.size()-1))
        {
            if(!locations[y+1][x+1].visited)
            {
                // not part of the loop
                possible_nodes.push_back({x+1, y+1});
            }
            else
            {
                //we don't evaluate diagonal walls
            }
        }

    }
    return possible_nodes;
}

int Elves::get_nest()
{
    //Note: visited locations are part of the loop
    int nest = 0;

    std::vector<coordinates_t> queue;

    coordinates_t current;
    
    replace_S();

    //we loop over all possible locations and possible edge nodes (no loop or possible exits)
    for(int y = 0; y < pipes.size(); y++)
    {
        for(int x = 0; x < pipes[y].length(); x++)
        {
            // we mark nodes at the border as exit if they are not part of the loop
            if((x == 0) || (x == pipes[y].length()-1) || (y == 0) || (y == pipes.size()-1))
            {
                if(!locations[y][x].visited)
                {
                    queue.push_back({x,y});
                }
                else
                {
                    // it is a loop node, only add L, J, 7, F
                    char symbol = pipes[y][x];
                    if(symbol == 'L' || symbol == 'J' || symbol == '7' || symbol == 'F')
                    {
                        queue.push_back({x,y});
                    }
                }
            }
            else
            {
                //non loop and non exit nodes are marked as nest
                if(!locations[y][x].visited)
                {
                    locations[y][x].nest = true;
                }
            }
        }
    }

    //all possible exit positions are in the queue -> we explore them all, all nodes which are reached are exit nodes and not nest
    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        if(locations[current.y][current.x].nest_visited)
        {
            continue;
        }

        //lets visit the location: visited nodes are reached from an exit node and are non nest
        locations[current.y][current.x].nest_visited = true;
        locations[current.y][current.x].nest = false;
        locations[current.y][current.x].exit = true;


        //check the neighbours
        std::cout << current.x << ", " << current.y << ":" << std::endl;
        locations[current.y][current.x].next_nest = get_next_nest(current.x, current.y);

        for(auto & l : locations[current.y][current.x].next_nest)
        {
            std::cout << "\t" << l.x << ", " << l.y << std::endl;
            if(!locations[l.y][l.x].nest_visited)
            {
                queue.push_back(l);
            }
        }
    }

    pipes[start.y][start.x] = 'S';

    print_nest();

    for(int y = 0; y < pipes.size(); y++)
    {
        for(int x = 0; x < pipes[y].length(); x++)
        {
            if(locations[y][x].nest)
            {
                nest++;
            }
        }
    }
    return nest;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<location_t> loc;

    std::vector<coordinates_t> next;

    int y = 0;
    std::size_t x = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            loc.resize(line.length());
            if(line.length() > 0)
            {         
                pipes.push_back(line);
                locations.push_back(loc);

                x = line.find('S');
                if(std::string::npos != x)
                {
                    start = {(int)x, y};
                }
            }
            y++;
        }

        file.close();
    }
}


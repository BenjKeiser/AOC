#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>
#include <cmath>

//part1
struct node_t {
    int x;
    int y;
    int steps;

    bool const operator<(const node_t &n) const 
    {
        if(x != n.x)
        {
            return x < n.x;
        }

        if(y != n.y)
        {
            return y < n.y;
        }

        if(steps != n.steps)
        {
            return steps < n.steps;
        }

        return false;
    }

    bool const operator> (const node_t &n) const
    { 
        return steps > n.steps; 
    }
};

std::vector<node_t> get_next_nodes(node_t node, std::vector<std::string> & plot)
{
    std::vector<node_t> nodes;
    int row = plot.size();
    int col = plot[0].length();
    int dx[] = { -1, 0, 1, 0 };
    int dy[] = { 0, 1, 0, -1 };

    for (int i = 0; i < 4; i++) 
    {
        int x = node.x + dx[i];
        int y = node.y + dy[i];
        if(y >= 0 && y < row && x >= 0 && x < col)
        {
            if(plot[y][x] != '#')
            {
                nodes.push_back({x, y, node.steps + 1});
            }
        }
    }
    return nodes;
}

uint64_t Elves::get_reachable_plots(int steps)
{
    uint64_t plots = 0;

    std::vector<node_t> queue;
    std::map<node_t,bool> visited_nodes;
    queue.push_back({start.first, start.second, 0});

    while(!queue.empty())
    {
        node_t current = queue.front();
        queue.erase(queue.begin());
        
        if(visited_nodes[current])
        {
            continue;
        }

        if(current.steps == steps)
        {
            plots++;
        }
        else
        {
            std::vector<node_t> next = get_next_nodes(current, garden_plot);
            queue.insert(queue.end(), next.begin(), next.end());
        }
        visited_nodes[current] = true;
    }

    return plots;
}

//part2
uint64_t Elves::get_reachable_plots_looped(int steps)
{
    int row = garden_plot.size();  
    int col = garden_plot[0].length();
    uint64_t p2TargetStepCount = steps;
    
    std::vector<std::vector<int>> distances;

    for(int y = 0; y < row; y++)
    {
        std::vector<int> dist;
        for(int x = 0; x < col; x++)
        {
            dist.push_back(INT32_MAX);
        }
        distances.push_back(dist);
    }

    // Counters we require for the maths
    uint64_t p1Count = 0;   //for verification
    uint64_t p2CenterOnParityCount = 0;
    uint64_t p2CenterOffParityCount = 0;
    uint64_t p2OtherDiamondBothParityCount = 0;

    std::priority_queue<node_t, std::vector<node_t>, std::greater<node_t>> queue;
    queue.push({start.first, start.second, 0});

    while (!queue.empty())
    {
        node_t current = queue.top();
        queue.pop();

        if (distances[current.y][current.x] != INT32_MAX)
        {
            continue;
        }

        distances[current.y][current.x] = current.steps;

        if (current.steps <= 64)
        {
            // Count up all the step lengtsh <= 64 for part 1
            if ((current.steps & 1) == 0)
            { 
                p1Count++; 
            }
        }

        // Now tally up the part 2 counts (more on what's going on here after this loop)
        if (current.steps <= row / 2)
        {
            // For part 2, there's a diamond pattern, count everything inside the diamond (manhattan distance of
            //  width / 2) at each even/odd parity
            if ((current.steps & 1) == (p2TargetStepCount & 1))
            {
                p2CenterOnParityCount++; 
            }
            else
            {
                p2CenterOffParityCount++;
            }
        }
        else
        {
            // Outside of the diamond, we care about the corners, but because every even/odd parity corner piece comes in
            //  pairs, we'll just tally up literally all the walkable space in those outer portions.
            p2OtherDiamondBothParityCount++;
        }

        // Queue up any neighboring directions that are on-grid.
        std::vector<node_t> next = get_next_nodes(current, garden_plot);
        for(auto & p : next)
        {
            queue.push(p);
        }

    }

    // In the actual puzzle input (although not the example, blah), the input has diamond channels cut in it:
    //
    //  AAAA.E.BBBB
    //  AAA.EEE.BBB
    //  AA.EEEEE.BB
    //  A.EEEEEEE.B
    //  .EEEEEEEEE.
    //  C.EEEEEEE.D
    //  CC.EEEEE.DD
    //  CCC.EEE.DDD
    //  CCCC.E.DDDD
    //
    // We're going to count up how many reachable odd-step squares there are in the E segment
    //  (p2CenterOnParityCount), how many reachable even-step squares there are "p2CenterOffParityCount", and then
    //  how many of both odds and evens we can touch in the remaining regions. More on that later.

    // Every grid.Width steps we add an additional ring of the diamonds we've measured to the list.
    const uint64_t stepExtent = p2TargetStepCount / col;

    // Since the initial step size is half the grid width (65 in the actual input), we count up all of the odd
    //  reachable squares in the E region of the above diagram at the center.
    // If we add another grid width worth of step (65 + 131 = 196) we'd be looking at these regions
    //  (extra spaces added between grid squares to make them easier to make out, "-" means "not counted", and
    //  lowercase are off-parity (i.e. count the evens instead of the odds):
    // ----.-.---- ----.e.---- ----.-.----
    // ---.---.--- ---.eee.--- ---.---.---
    // --.-----.-- --.eeeee.-- --.-----.--
    // -.-------.- -.eeeeeee.- -.-------.-
    // .---------. .eeeeeeeee. .---------.
    // ----------- eeeeeeeeeee -----------
    // .---------. .eeeeeeeee. .---------.
    // -.-------.D c.eeeeeee.d C.-------.-
    // --.-----.DD cc.eeeee.dd CC.-----.--
    // ---.---.DDD ccc.eee.ddd CCC.---.---
    // ----.-.DDDD cccc.e.dddd CCCC.-.----

    // ----.e.bbbb AAAA.E.BBBB aaaa.e.----
    // ---.eee.bbb AAA.EEE.BBB aaa.eee.---
    // --.eeeee.bb AA.EEEEE.BB aa.eeeee.--
    // -.eeeeeee.b A.EEEEEEE.B a.eeeeeee.-
    // .eeeeeeeee. .EEEEEEEEE. .eeeeeeeee.
    // eeeeeeeeeee EEEEEEEEEEE eeeeeeeeeee
    // .eeeeeeeee. .EEEEEEEEE. .eeeeeeeee.
    // -.eeeeeee.D C.EEEEEEE.D C.eeeeeee.-
    // --.eeeee.DD CC.EEEEE.DD CC.eeeee.--
    // ---.eee.DDD CCC.EEE.DDD CCC.eee.---
    // ----.e.DDDD CCCC.E.DDDD CCCC.e.----

    // ----.-.BBBB aaaa.e.bbbb AAAA.-.----
    // ---.---.BBB aaa.eee.bbb AAA.---.---
    // --.-----.BB aa.eeeee.bb AA.-----.--
    // -.-------.B a.eeeeeee.b A.-------.-
    // .---------. .eeeeeeeee. .---------.
    // ----------- eeeeeeeeeee -----------
    // .---------. .eeeeeeeee. .---------.
    // -.-------.- -.eeeeeee.- -.-------.-
    // --.-----.-- --.eeeee.-- --.-----.--
    // ---.---.--- ---.eee.--- ---.---.---
    // ----.-.---- ----.e.---- ----.-.----

    // in that ring we end up adding:
    //  4x "e" regions (count the evens)
    //  2x "A", "a", "B", "b", "C", "c", "D", and "d" regions (equal parts of each, of both parities)
    //
    // If we add another ring (not going to draw that diagram), we end up adding:
    //  8x "E" regions (count the odds again)
    //  4x "A", "a", "B", "b", "C", "c", "D", and "d" regions (again equal parts of each, both parities)
    //
    // Next ring:
    // 12x "e" regions (back to the evens)
    //  6x "A", "a", "B", "b", "C", "c", "D", and "d" regions (same as before, equal of all)
    //
    // So every Nth multiple of gridWidth we add, we add N*4 "E" or "e" regions (alternating) and N*2 of the other
    //  regions (which is the same as saying we add N*2 of an "AbCd" diamond and N*2 of an "aBcD" diamond.
    //
    // The closed form of each of those (taking advantage of integer division for the center counts)

    // ECount (count of "E" regions): = 1 1 9 9 25 25 49 49 ...
    uint64_t ECount = std::pow(2*(stepExtent / 2) + 1, 2);

    // eCount (count of "e" regions): = 0 4 4 16 16 36 36 64 64 ...
    uint64_t eCount = std::pow(2*((stepExtent + 1) / 2), 2);

    // aAbBcCdDCount (count of all other regions) = 0 2 6 12 20 30 42 ...
    uint64_t aAbBcCdDCount = (ECount + eCount) / 2;

    // Summing allllll that up (multiplied by the values in each square) gives us:
    uint64_t p2Count = ECount * p2CenterOnParityCount + eCount * p2CenterOffParityCount + aAbBcCdDCount * p2OtherDiamondBothParityCount;

    return p2Count;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;
    
    int x = 0;
    int y = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            garden_plot.push_back(line);
            x = line.find('S');
            if(std::string::npos != x)
            {
                start = {x, y};
            }
            y++;
        }
        std::cout << std::endl;
        file.close();
    }
}


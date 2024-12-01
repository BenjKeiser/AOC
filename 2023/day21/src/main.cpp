#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    int steps = 6;
    
    if(argc >= 3)
    {
        steps = std::stoi(argv[2]);
    }
    std::cout << steps << " steps: " << elves.get_reachable_plots(steps) << " plots" << std::endl;

    steps = 26501365;
    std::cout << steps << " steps: " << elves.get_reachable_plots_looped(steps) << " plots" << std::endl;
    return 0;
}
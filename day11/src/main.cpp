#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_shortest_paths(2) << std::endl;
    std::cout << elves.get_shortest_paths(10) << std::endl;
    std::cout << elves.get_shortest_paths(100) << std::endl;
    std::cout << elves.get_shortest_paths(1000000) << std::endl;
    return 0;
}
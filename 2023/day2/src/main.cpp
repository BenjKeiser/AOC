#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    //std::cout << std::to_string(elves.get_games()) << std::endl;
    std::cout << std::to_string(elves.get_balls()) << std::endl;
    return 0;
}
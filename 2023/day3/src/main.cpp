#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_parts() << std::endl;
    std::cout << elves.get_gears() << std::endl;
    return 0;
}
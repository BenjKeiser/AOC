#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_closest_location() << std::endl;
    std::cout << elves.get_closest_location2() << std::endl;
    return 0;
}
#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_lagoon() << std::endl;
    std::cout << elves.get_lagoon_colour() << std::endl;
    return 0;
}
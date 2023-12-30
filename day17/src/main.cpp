#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_heat_loss(1, 3) << std::endl;
    std::cout << elves.get_heat_loss(4, 10) << std::endl;
    return 0;
}
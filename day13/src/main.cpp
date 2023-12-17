#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_mirrors(0) << std::endl;
    std::cout << elves.get_mirrors(1) << std::endl;
    return 0;
}
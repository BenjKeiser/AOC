#include "elf.h"

#include <iostream>
#include <string>


int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_arrangements(1) << std::endl;
    std::cout << elves.get_arrangements(5) << std::endl;
    return 0;
}
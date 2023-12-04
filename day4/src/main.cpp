#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_value() << std::endl;
    std::cout << elves.get_cards() << std::endl;
    return 0;
}
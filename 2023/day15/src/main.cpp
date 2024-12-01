#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_hash_sum() << std::endl;
    std::cout << elves.get_focus_power() << std::endl;
    return 0;
}
#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.eval_races() << std::endl;
    std::cout << elves.eval_race_p2() << std::endl;
    return 0;
}
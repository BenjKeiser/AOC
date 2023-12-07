#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1], false);
    std::cout << elves.get_winnings() << std::endl;
    Elves elves_j(argv[1], true);
    std::cout << elves_j.get_winnings() << std::endl;
    //std::cout << elves.eval_race_p2() << std::endl;
    return 0;
}
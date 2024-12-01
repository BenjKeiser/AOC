#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.eval_history_next() << std::endl;
    std::cout << elves.eval_history_before() << std::endl;
    return 0;
}
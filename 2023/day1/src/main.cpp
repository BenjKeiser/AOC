#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    int pairs = 0;
    Elves elves(argv[1]);
    std::cout << std::to_string(elves.get_calib_data()) << std::endl;
    return 0;
}
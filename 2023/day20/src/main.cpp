#include "elf.h"

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
    Elves elves(argv[1]);
    std::cout << elves.get_pulses(1000) << std::endl;
    std::cout << elves.get_rx_pulse() << std::endl;
    return 0;
}
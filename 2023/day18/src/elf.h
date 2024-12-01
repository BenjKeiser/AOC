#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

enum direction_t {
    R=0,
    D=1,
    L=2,
    U=3,
    F=4
};

struct dig_t {
    direction_t direction;
    int length;
    std::string colour;
};

class Elves {
    private:
        std::vector<dig_t> dig_orders;

    public:
        Elves(char * file_name);
        uint64_t get_lagoon();
        uint64_t get_lagoon_colour();

};

#endif /* ELF_H */



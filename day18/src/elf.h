#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

struct dig_t {
    char direction;
    int length;
    std::string colour;
};

class Elves {
    private:
        std::vector<dig_t> dig_orders;

    public:
        Elves(char * file_name);
        uint64_t get_lagoon();

};

#endif /* ELF_H */



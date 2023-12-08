#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

struct location_t 
{
    std::string loc;
    std::string left;
    std::string right;
};

class Elves {
    private:
        std::string steps;
        std::vector<location_t> locations;
    public:
        Elves(char * file_name);
        int get_moves();
        int64_t get_moves_ghost();
};

#endif /* ELF_H */



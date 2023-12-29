#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Elves {
    private:
        std::vector<std::vector<int>> heat_map;
        uint64_t dijkstra();
    public:
        Elves(char * file_name);
        uint64_t get_heat_loss();
};

#endif /* ELF_H */



#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

enum spring_t {
    UNKNOWN=0,      //?
    OPERATIONAL=1,  //.
    DAMAGED=2       //#
};

struct row_t {
    std::vector<spring_t> spring_list;
    std::vector<int> damaged_groups;
};

class Elves {
    private:
        std::vector<row_t> all_springs;
        spring_t symbol_to_spring(char symbol);
        uint64_t get_arrangement(row_t row);
    public:
        Elves(char * file_name);
        uint64_t get_arrangements();
};

#endif /* ELF_H */



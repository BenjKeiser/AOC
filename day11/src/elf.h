#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Elves {
    private:
        std::vector<std::string> galaxy;
    public:
        Elves(char * file_name);
        int64_t get_shortest_paths(int factor);
};

#endif /* ELF_H */



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
        int get_shortest_paths();
};

#endif /* ELF_H */



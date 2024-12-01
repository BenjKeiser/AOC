#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Elves {
    private:
        std::vector<std::vector<std::string>> patterns;
    public:
        Elves(char * file_name);
        int get_mirrors(int smudge_count);
};

#endif /* ELF_H */



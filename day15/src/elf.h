#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Elves {
    private:
        std::vector<std::string> steps;

        unsigned char get_hash(std::string str);

    public:
        Elves(char * file_name);
        uint64_t get_hash_sum();
};

#endif /* ELF_H */



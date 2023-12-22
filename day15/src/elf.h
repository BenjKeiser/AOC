#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <unordered_map>



class Elves {
    private:
        std::vector<std::string> steps;
        std::vector<std::unordered_map<std::string, int>> boxes;

        unsigned char get_hash(std::string str);

    public:
        Elves(char * file_name);
        uint64_t get_hash_sum();
        uint64_t get_focus_power();

};

#endif /* ELF_H */



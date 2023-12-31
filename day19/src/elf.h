#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <unordered_map>
#include <iostream>


struct limits_t {
    int x_min;
    int x_max;
    int m_min;
    int m_max;
    int a_min;
    int a_max;
    int s_min;
    int s_max;

    void print()
    {
        std::cout << x_min << ", " << x_max << std::endl;
        std::cout << m_min << ", " << m_max << std::endl;
        std::cout << a_min << ", " << a_max << std::endl;
        std::cout << s_min << ", " << s_max << std::endl;
    }
};

struct rule_t {
    char id;
    char op;
    int val;
    std::string go;
};

struct filter_t {
    std::vector<rule_t> rules;
    std::string def;
};

struct part_t {
    int x;
    int m;
    int a;
    int s;

    int rating;
    bool accepted;
};

class Elves {
    private:
        std::unordered_map<std::string, filter_t> filters;
        std::vector<part_t> parts;
        void run_filter(part_t * part, std::string filter);
        std::pair<bool, limits_t> check_filter(limits_t limits, std::string filter);
    public:
        Elves(char * file_name);
        uint64_t get_parts();
        uint64_t get_combinations();

};

#endif /* ELF_H */



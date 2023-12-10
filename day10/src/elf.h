#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <map>

enum direction_t {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3
};

struct coordinates_t {
    int x;
    int y;
};

struct location_t {
    int steps;
    bool visited;

    bool nest_visited; // visited for nest search
    bool nest; // part of the nest
    bool exit; // there is an exit here -> no nest

    std::vector<coordinates_t> next;
    std::vector<coordinates_t> next_nest;
};

class Elves {
    private:
        std::vector<std::string> pipes;
        std::vector<std::vector<location_t>> locations;
        coordinates_t start;
    public:
        Elves(char * file_name);
        std::vector<coordinates_t> get_next(int x, int y);
        std::vector<coordinates_t> get_next_nest(int x, int y);
        int get_farthest();
        int get_nest();

        void print();
        void print_nest();
};

#endif /* ELF_H */



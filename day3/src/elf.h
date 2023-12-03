#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

struct gear_t {
    int x;
    int y;
};

struct symbol_t {
    int x;
    int y;
};

struct part_t {
    int nb;
    int x_s;
    int x_e;
    int y;
};

class Engine {
    private:
        std::vector<std::vector<gear_t>> gears;
        std::vector<std::vector<symbol_t>> symbols;
        std::vector<std::vector<part_t>> parts;
        int symbol_lines;
        int part_lines;
        int gear_lines;
    public:
        Engine();
        void add_part(part_t part);
        void add_symbol(symbol_t symbol);
        void add_gear(gear_t gear);
        int get_parts();
        int get_gears();
};

class Elves {
    private:
        Engine engine;
    public:
        Elves(char * file_name);
        int get_parts();
        int get_gears();
};

#endif /* ELF_H */



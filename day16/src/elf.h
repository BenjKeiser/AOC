#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

enum direction_t {
    UP=0,
    DOWN=1,
    RIGHT=2,
    LEFT=3
};

struct move_t {
    int x;
    int y;
    direction_t dir;
};

struct location_t {
    bool visited[4];

    bool is_energized()
    {
        if(visited[UP] || visited[DOWN] || visited[LEFT] || visited[RIGHT])
        {
            return true;
        }
        else
        {
            return false;
        }
    }
};

class Elves {
    private:
        std::vector<std::vector<location_t>> locations;
        std::vector<std::string> grid;
        std::vector<move_t> get_next_moves(move_t move);

    public:
        Elves(char * file_name);
        uint64_t get_energized(move_t start);

};

#endif /* ELF_H */



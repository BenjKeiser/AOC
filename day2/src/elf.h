#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

struct game_t {
    int red;
    int green;
    int blue;
};

class Game {
    private:
        std::vector<game_t> games;
        int idx;
    public:
        void add_game(game_t game);
        int is_possible(game_t balls);
        int get_balls();
        Game(int index);
};

class Elves {
    private:
        std::vector<Game> games;
    public:
        Elves(char * file_name);
        int get_games();
        int get_balls();
};

#endif /* ELF_H */



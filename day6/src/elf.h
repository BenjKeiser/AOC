#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

struct race_t {
    int64_t time;
    int64_t dist;
};

class Elves {
    private:
        std::vector<race_t> races;
        int64_t p2_time;
        int64_t p2_dist;
    public:
        Elves(char * file_name);
        int eval_races();
        int eval_race_p2();
};

#endif /* ELF_H */



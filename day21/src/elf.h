#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>

class Elves {
    private:
        std::vector<std::string> garden_plot;
        std::pair<int,int> start;

    public:
        Elves(char * file_name);
        uint64_t get_reachable_plots(int steps);
        uint64_t get_reachable_plots_looped(int steps);
};

#endif /* ELF_H */



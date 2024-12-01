#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Elves {
    private:
        std::vector<std::vector<int>> history;
    public:
        Elves(char * file_name);
        int get_next(std::vector<int> history);
        int get_before(std::vector<int> history);
        int64_t eval_history_next();
        int64_t eval_history_before();
};

#endif /* ELF_H */



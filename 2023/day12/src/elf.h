#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <map>
#include <iostream>

enum spring_t {
    UNKNOWN=0,      //?
    OPERATIONAL=1,  //.
    DAMAGED=2       //#
};

struct row_t {
    std::vector<spring_t> spring_list;
    std::vector<int> damaged_groups;
    int index;

    bool const operator==(const row_t &r) const {
        if(index != r.index)
        {
                return false;
        }
        if(damaged_groups.size() == r.damaged_groups.size())
        {
            if(damaged_groups != r.damaged_groups)
            {
                return false;
            }
        }
        else
        {
            return false;
        }
        return true;
    }

    bool const operator<(const row_t &r) const {
        if(index != r.index)
        {
            return index < r.index;
        }

        if(damaged_groups.size() != r.damaged_groups.size())
        {
            return damaged_groups.size() < r.damaged_groups.size();
        }
        else
        {
            for(int i = 0; i < damaged_groups.size(); i++)
            {
                if(damaged_groups[i] != r.damaged_groups[i])
                {
                    return damaged_groups[i] < r.damaged_groups[i];
                }
            }
        }

        return false;
    }
};

#if 0
auto comp = [](const row_t & a, const row_t & b) 
{ 
    return a < b; 
};
#endif

class Elves {
    private:
        std::map<row_t, uint64_t> results;
        std::vector<row_t> all_springs;
        spring_t symbol_to_spring(char symbol);
        uint64_t get_arrangement(row_t row);
    public:
        Elves(char * file_name);
        uint64_t get_arrangements(int factor);
};

#endif /* ELF_H */



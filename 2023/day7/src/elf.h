#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

enum val_t {
    HIGHEST=0,
    ONE_PAIR,
    TWO_PAIRS,
    THREE,
    FULL_HOUSE,
    FOUR,
    FIVE
};

struct hand_t {
    std::string hand;
    val_t val;
    val_t val_j;
    int bid;
};

class Elves {
    private:
        std::vector<hand_t> hands;
        val_t eval_hand(hand_t hand);
        val_t eval_hand_jokers(hand_t hand);
        int card_to_int(char card);
        bool js; 
    public:
        Elves(char * file_name, bool jokers);
        int get_winnings();
};

#endif /* ELF_H */



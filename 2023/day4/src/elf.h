#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>


class Card {
    private:
        std::vector<int> winners;
        std::vector<int> numbers;
        int card;
        int amount;
    public:
        Card(int nb, std::vector<int> w, std::vector<int> n);
        int get_value();
        int get_matches();
        int get_amount();
        void add_amount(int a);
};

class Elves {
    private:
        std::vector<Card> cards;
    public:
        Elves(char * file_name);
        int get_value();
        int get_cards();
};

#endif /* ELF_H */



#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

val_t get_val(std::vector<int> counts)
{
    val_t val = HIGHEST;

    if(counts[0] == 5)
    {
        val = FIVE;
    }
    else if(counts[0] == 4)
    {
        val = FOUR;
    }
    else if(counts[0] == 3 && counts[1] == 2)
    {
        val = FULL_HOUSE;
    }
    else if(counts[0] == 3 && counts[1] != 2)
    {
        val = THREE;
    }
    else if(counts[0] == 2 && counts[1] == 2)
    {
        val = TWO_PAIRS;
    }
    else if(counts[0] == 2 && counts[1] !=2)
    {
        val = ONE_PAIR;
    }
    else
    {
        val = HIGHEST;    
    }

    return val;
}

val_t Elves::eval_hand(hand_t hand)
{
    std::vector<int> counts;
    std::string::difference_type n = 0;
    val_t val = HIGHEST;
    
    sort(hand.hand.begin(), hand.hand.end());

    for(int i = 0; i < hand.hand.length(); i++)
    {
        n = std::count(hand.hand.begin(), hand.hand.end(), hand.hand[i]);
        counts.push_back(n);
        i += n - 1;        
    }

    sort(counts.begin(), counts.end(), std::greater<int>());

    return get_val(counts);
}

val_t Elves::eval_hand_jokers(hand_t hand)
{
    std::vector<int> counts;
    std::string::difference_type n = 0;
    val_t val = HIGHEST;

    int jokers = 0;
    
    sort(hand.hand.begin(), hand.hand.end());

    for(int i = 0; i < hand.hand.length(); i++)
    {
        n = std::count(hand.hand.begin(), hand.hand.end(), hand.hand[i]);
        if(hand.hand[i] == 'J')
        {
            jokers = n;
        }
        else
        {
            counts.push_back(n);
        }
        i += n-1;        
    }

    if(counts.size() == 0)
    {
        counts.push_back(jokers);
        jokers = 0;
    }

    sort(counts.begin(), counts.end(), std::greater<int>());

    if(jokers)
    {
        counts[0] += jokers;
    }

    return get_val(counts);
}

int card_to_int(char card)
{
    int val = 0;
    if(card < 0x41)
    {
        val = card - '0';
    }
    else if (card == 'T')
    {
        val = 10;
    }
    else if (card == 'J')
    {
        val = 11;
    }
    else if (card == 'Q')
    {
        val = 12;
    }
    else if (card == 'K')
    {
        val = 13;
    }
    else if (card == 'A')
    {
        val = 14;
    }

    return val;
}

bool compare_hand(hand_t h1, hand_t h2) 
{ 
    bool v = true;
    int c1 = 0;
    int c2 = 0;

    if(h1.val == h2.val)
    {
        for(int i = 0; i < h1.hand.length(); i++)
        {
            if(h1.hand[i] != h2.hand[i])
            {
                v = card_to_int(h1.hand[i]) < card_to_int(h2.hand[i]);
                break;
            }
        }
    }
    else
    {
        v = h1.val < h2.val;
    }
    return v; 
} 

int Elves::get_winnings()
{
    int sum = 0;
    std::vector<hand_t> hs;
    hs = hands;
    sort(hs.begin(), hs.end(), compare_hand);
    
    for(int i = 0; i < hs.size(); i++)
    {
        sum += (i+1)*hs[i].bid;
    }
    return sum;
}

Elves::Elves(char * file_name, bool jokers)
{
    std::ifstream file(file_name);
    std::string line;

    js = jokers;
    hand_t h;
    int pos = 0;
    int nb = 0;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            h.hand = line.substr(0, 5);
            h.bid = std::stoi(line.substr(6));

            if(!js)
            {
                h.val = eval_hand(h);
            }
            else
            {
                h.val = eval_hand_jokers(h);
                std::replace( h.hand.begin(), h.hand.end(), 'J', '1');
            }

            hands.push_back(h);
        }
        file.close();
    }
}


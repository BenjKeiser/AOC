#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

Card::Card(int nb, std::vector<int> w, std::vector<int> n)
{
    card = nb;
    winners = w;
    numbers = n;
    amount = 1;
}


int Card::get_amount()
{
    return amount;
}

void Card::add_amount(int a)
{
    amount += a;
}

int Card::get_value()
{
    int val = 0;
    for(auto & w : winners)
    {
        for(auto & n : numbers)
        {
            if(w == n)
            {
                if(val == 0)
                {
                    val = 1;
                }
                else
                {
                    val *= 2;
                }
            }
        }
    }
    return val;
}

int Card::get_matches()
{
    int val = 0;
    for(auto & w : winners)
    {
        for(auto & n : numbers)
        {
            if(w == n)
            {
                val++;
            }
        }
    }
    return val;
}

int Elves::get_cards()
{
    int sum = 0;
    int match = 0;
    int amount = 0;
    int idx = 0;
    for(int c = 0; c < cards.size(); c++)
    {
        match = cards[c].get_matches();
        amount = cards[c].get_amount();
        sum += amount;
        for(int i = c + 1; i <= c + match; i++)
        {
            cards[i].add_amount(amount);
        }

    }
    return sum;
}

int Elves::get_value()
{
    int val = 0;
    for(auto & c : cards)
    {
        val += c.get_value();
    }
    return val;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<int> winners;
    std::vector<int> numbers;

    int card = 0;
    int pos = 0;
    int nb = 0;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            //Card Number
            winners.clear();
            numbers.clear();
            card = std::stoi(line.substr(5));
            std::cout << "Card: " << std::to_string(card) << std::endl;

            //Winning Numbers
            pos = line.find(":");
            std::istringstream ws(line.substr(pos+1));
            std::cout << "Winners: ";
            while (ws >> nb)
            {
                winners.push_back(nb);
                std::cout << std::to_string(nb) << " ";
            }
            std::cout << std::endl;

            //Card Numbers
            pos = line.find("|");
            std::istringstream ns(line.substr(pos+1));
            std::cout << "Numbers: ";
            while (ns >> nb)
            {
                numbers.push_back(nb);
                std::cout << std::to_string(nb) << " ";
            }
            std::cout << std::endl;
            Card c(card, winners, numbers);
            cards.push_back(c);
        }
        file.close();
    }
}


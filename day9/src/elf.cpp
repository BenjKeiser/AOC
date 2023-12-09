#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>


int Elves::get_next(std::vector<int> history)
{
    std::vector<std::vector<int>> sequence;
    std::vector<int> s; 

    int diff = 0;
    int val = 0;

    sequence.push_back(history);

    while(history.size() >= 2)
    {
        //fill sequence
        s.clear();
        for(int i = 0; i < history.size() - 1; i++)
        {
            diff = history[i+1] - history[i];
            s.push_back(diff);
        }
        sequence.push_back(s);
        history = s;

        if( s.size() == std::count(s.begin(), s.end(), 0))
        {
            //all 0, we can go back
            diff = 0;
            while(!sequence.empty())
            {
                s = sequence.back();
                sequence.pop_back();
                diff += s.back();
            }
            break;
        }
    }
    return diff;
}

int Elves::get_before(std::vector<int> history)
{
    std::vector<std::vector<int>> sequence;
    std::vector<int> s; 

    int diff = 0;
    int val = 0;

    sequence.push_back(history);

    while(history.size() >= 2)
    {
        //fill sequence
        s.clear();
        for(int i = 0; i < history.size() - 1; i++)
        {
            diff = history[i+1] - history[i];
            s.push_back(diff);
        }
        sequence.push_back(s);
        history = s;

        if( s.size() == std::count(s.begin(), s.end(), 0))
        {
            //all 0, we can go back
            diff = 0;
            while(!sequence.empty())
            {
                s = sequence.back();
                sequence.pop_back();
                diff = s.front() - diff;
            }
            break;
        }
    }
    return diff;
}


int64_t Elves::eval_history_next()
{
    int64_t sum = 0;
    int next = 0;
    for(auto & h : history)
    {
        next = get_next(h);
        sum += next;
    }
    return sum;
}

int64_t Elves::eval_history_before()
{
    int64_t sum = 0;
    int next = 0;
    for(auto & h : history)
    {
        next = get_before(h);
        sum += next;
    }
    return sum;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<int> h;
    int nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {         
                h.clear();       
                std::istringstream l(line);
                while(l >> nb)
                {
                    h.push_back(nb);
                }
                history.push_back(h);
            }
        }

        for(auto & h : history)
        {
            for(auto & i : h)
            {
                std::cout << i << " ";
            }
            std::cout << std::endl;
        }
        file.close();
    }
}


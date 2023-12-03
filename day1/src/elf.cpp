#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                calibration.push_back(line);
            }
        }
        file.close();
    }
}

void Elves::replace_digits(std::string * line)
{
    int pos = 0;
    bool found = false;
    std::string digits[9] = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    for(int i = 0; i < 9; i++)
    {
        do 
        {
            found = false;
            pos = line->find(digits[i]);
            if (pos != std::string::npos)
            {
                line->replace(pos+1, digits[i].length()-2, std::to_string(i+1));
                found = true;
            }
        } while(found != false);       
    }
}

char Elves::get_first_digit(std::string calib)
{
    char digit = 0;
    for(int i = 0; i < calib.length(); i++)
    {
        if(calib[i] >= 0x30 && calib[i] <= 0x39)
        {
            digit = calib[i];
            break;
        }
    }
    return digit;
}

char Elves::get_last_digit(std::string calib)
{
    char digit = 0;
    for(int i = calib.length()-1; i >= 0; i--)
    {
        if(calib[i] >= 0x30 && calib[i] <= 0x39)
        {
            digit = calib[i];
            break;
        }
    }
    return digit;
}

int Elves::get_calib_data()
{
    std::string calib;
    std::vector<int> data;
    int sum = 0;
    for (auto & c : calibration)
    {
        calib.clear();
        replace_digits(&c);
        calib.push_back(get_first_digit(c));
        calib.push_back(get_last_digit(c));
        std::cout << c << ": " << calib << std::endl;
        sum += stoi(calib);
    }
    return sum;
}

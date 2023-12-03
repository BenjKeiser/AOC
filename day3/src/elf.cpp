#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>

void Engine::add_part(part_t part)
{
    std::cout << "Add part: " << std::to_string(part.nb) << ", " << std::to_string(part.x_s) << ", " << std::to_string(part.x_e) << ", " << std::to_string(part.y) << std::endl; 
    std::vector<part_t> dummy;
    for(int i = part_lines; i < part.y + 1; i++)
    {
        parts.push_back(dummy);
        part_lines++;
        std::cout << "adding new part line" << std::endl;
    }
    parts[part.y].push_back(part);
}

void Engine::add_symbol(symbol_t symbol)
{
    std::cout << "Add symbol: " << std::to_string(symbol.x) << ", " << std::to_string(symbol.y) << std::endl; 
    std::vector<symbol_t> dummy;
    for(int i = symbol_lines; i < symbol.y + 1; i++)
    {
        symbols.push_back(dummy);
        symbol_lines++;
        std::cout << "adding new symbol line" << std::endl;
    }
    symbols[symbol.y].push_back(symbol);
}


void Engine::add_gear(gear_t gear)
{
    std::cout << "Add gear: " << std::to_string(gear.x) << ", " << std::to_string(gear.y) << std::endl; 
    std::vector<gear_t> dummy;
    for(int i = gear_lines; i < gear.y + 1; i++)
    {
        gears.push_back(dummy);
        gear_lines++;
        std::cout << "adding new gear line" << std::endl;
    }
    gears[gear.y].push_back(gear);
}

Engine::Engine()
{
    symbol_lines = 0;
    part_lines = 0;
}

int Engine::get_parts()
{
    int part_sum = 0;
    for(int lp = 0; lp < part_lines; lp++)
    {
        for(auto & p : parts[lp])
        {
            for(int ls = lp - 1; ls <= lp + 1; ls++ )
            {
                if(ls < 0 || ls >= symbol_lines)
                {
                    continue;
                }
                std::cout << "part line: " << std::to_string(lp) << ", symbol line: " << std::to_string(ls) << std::endl;
                for(auto & s : symbols[ls])
                {
                    if((p.x_s == s.x + 1) || (p.x_e == s.x - 1) || ((s.x >= p.x_s) && (s.x <= p.x_e)))
                    {
                        part_sum += p.nb;
                        std::cout << "adding part: " << p.nb << std::endl;
                    }
                }
            }
        }
    }
    return part_sum;
}

int Engine::get_gears()
{
    int gear_sum = 0;
    int found = 0;
    int gr = 0;
    for(int lg = 0; lg < gear_lines; lg++)
    {
        for(auto & g : gears[lg])
        {
            found = 0;
            for(int lp = lg - 1; lp <= lg + 1; lp++ )
            {
                if(lp < 0 || lp >= part_lines)
                {
                    continue;
                }
                std::cout << "gear line: " << std::to_string(lg) << ", part line: " << std::to_string(lp) << std::endl;
                for(auto & p : parts[lp])
                {
                    if((p.x_s == g.x + 1) || (p.x_e == g.x - 1) || ((g.x >= p.x_s) && (g.x <= p.x_e)))
                    {
                        found++;
                        if (found == 1)
                        {
                            gr = p.nb;
                        }
                        else if (found == 2)
                        {
                            gr = gr * p.nb;
                        }
                    }
                }
            }
            if(found == 2)
            {
                gear_sum += gr;
            }
        }
    }
    return gear_sum;
}

int Elves::get_parts()
{
    return engine.get_parts();
}

int Elves::get_gears()
{
    return engine.get_gears();
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::string::size_type idx;

    int line_nb = 0;
    int part_nb = 0;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {

            for(int i = 0; i < line.length(); i++)
            {
                // part
                if(line[i] >= 0x30 && line[i] <= 0x39)
                {
                    part_nb = std::stoi(line.substr(i), &idx);
                    engine.add_part({part_nb, i, i + idx - 1, line_nb});
                    i = i + idx - 1;
                    continue;
                }

                // symbol
                if((line[i] >= 0x21 && line[i] <= 0x2F && line[i] != '.') || (line[i] >= 0x3a && line[i] <= 0x40) || (line[i] >= 0x5b && line[i] <= 0x60) || (line[i] >= 0x7b && line[i] <= 0x7e))
                {
                    
                    engine.add_symbol({i, line_nb});
                }

                // gear
                if(line[i] == '*')
                {
                    engine.add_gear({i, line_nb});
                }
            }
            line_nb++;
        }
        file.close();
    }
}


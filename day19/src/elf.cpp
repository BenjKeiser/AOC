#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

//part 1
void Elves::run_filter(part_t * part, std::string filter)
{
    filter_t current = filters[filter];

    if(filter == "A")
    {
        part->accepted = true;
    }
    else if(filter == "R")
    {
        part->accepted = false;
    }
    else
    {
        int val = 0;

        std::string fun = current.def;

        for(auto & f : current.rules)
        {
            if(f.id == 'x')
            {
                val = part->x;
            }
            else if(f.id == 'm')
            {
                val = part->m;
            }
            else if(f.id == 'a')
            {
                val = part->a;
            }
            else if(f.id == 's')
            {
                val = part->s;
            }
            else
            {
                std::cout << "Huh -> " << f.id << std::endl;
                std::exit(-1);
            }

            if(f.op == '>')
            {
                if(val > f.val)
                {
                    fun = f.go;
                    break;
                }
            }
            else if(f.op == '<')
            {
                if(val < f.val)
                {
                    fun = f.go;
                    break;
                }
            }
            else
            {
                std::cout << "Huh -> " << f.op << std::endl;
                std::exit(-1);
            }
        }
        
        run_filter(part, fun);
    }
}

uint64_t Elves::get_parts()
{
    uint64_t sum = 0;

    for(auto & p : parts)
    {
        run_filter(&p, "in");
        sum += p.accepted ? p.rating : 0;
    }

    return sum;
}

//part 2
limits_t get_limits(limits_t l1, limits_t l2)
{
    limits_t limits;

    limits.x_min = std::max(l1.x_min, l2.x_min);
    limits.x_max = std::min(l1.x_max, l2.x_max);

    limits.m_min = std::max(l1.m_min, l2.m_min);
    limits.m_max = std::min(l1.m_max, l2.m_max);
    
    limits.a_min = std::max(l1.a_min, l2.a_min);
    limits.a_max = std::min(l1.a_max, l2.a_max);
    
    limits.s_min = std::max(l1.s_min, l2.s_min);
    limits.s_max = std::min(l1.s_max, l2.s_max);

    return limits;
}

std::pair<bool, limits_t> Elves::check_filter(limits_t limits, std::string filter)
{
    filter_t current = filters[filter];

    limits_t lim = limits;
    limits_t lim_next = lim;

    bool result = false;
    bool br = false;

    if(filter == "A")
    {
        return {true, limits};
    }
    else if(filter == "R")
    {
        return {false, limits};
    }
    else
    {
        int val = 0;

        std::string fun = current.def;

        for(auto & f : current.rules)
        {
            //todo: Set lim_next
            if(f.op == '>')
            {
                if(f.id == 'x')
                {
                    if(lim.x_min <= f.val)
                    {
                        lim.x_min = f.val + 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.x_max = std::min(f.val, lim.x_max);
                }
                else if(f.id == 'm')
                {
                    if(lim.m_min <= f.val)
                    {
                        lim.m_min = f.val + 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.m_max = std::min(f.val, lim.m_max);
                }
                else if(f.id == 'a')
                {
                    if(lim.a_min <= f.val)
                    {
                        lim.a_min = f.val + 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.a_max = std::min(f.val, lim.a_max);
                }
                else if(f.id == 's')
                {
                    if(lim.s_min <= f.val)
                    {
                        lim.s_min = f.val + 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.s_max = std::min(f.val, lim.s_max);
                }
            }
            else if(f.op == '<')
            {
                if(f.id == 'x')
                {
                    if(lim.x_max >= f.val)
                    {
                        lim.x_max = f.val - 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.x_min = std::max(f.val, lim.x_min);
                }
                else if(f.id == 'm')
                {
                    if(lim.m_max >= f.val)
                    {
                        lim.m_max = f.val - 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.m_min = std::max(f.val, lim.m_min);
                }
                else if(f.id == 'a')
                {
                    if(lim.a_max >= f.val)
                    {
                        lim.a_max = f.val - 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.a_min = std::max(f.val, lim.a_min);
                }
                else if(f.id == 's')
                {
                    if(lim.s_max >= f.val)
                    {
                        lim.s_max = f.val - 1;
                    }
                    else
                    {
                        //we will always end up here... stop after evaluating
                        br = true;
                    }
                    lim_next.s_min = std::max(f.val, lim.s_min);
                }
                
            }
            auto [r, l] = check_filter(lim, f.go);
            if(r)
            {
                limits = get_limits(l, limits);
                result = true;
                br = true;
            }
            // we need to ensure that the function is reached (they are executed sequentially)
            lim = lim_next;
            
            if(br)
            {
                //we've evaluated a node where we cannot go further
                break;
            }
        }

        if(!br)
        {
            //with these limits we will enter the default case
            auto [r, l] = check_filter(lim, current.def);
            if(r)
            {
                limits = get_limits(l, limits);
                result = true;
            }    
        }
    }
    return {result, limits};
}

uint64_t Elves::get_combinations()
{
    uint64_t comb = 0;
    limits_t limits = {0, 4000, 0, 4000, 0, 4000, 0, 4000};

    auto [res, l] = check_filter(limits, "in");

    if(res)
    {
        l.print();
        comb = (uint64_t)(l.x_max - l.x_min) * (uint64_t)(l.m_max - l.m_min) * (uint64_t)(l.a_max - l.a_min) * (uint64_t)(l.s_max - l.s_min);
    } 
    else
    {
        std::cout << "Huh...." << std::endl;
    }

    return comb;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    size_t pos;
    size_t next;
    std::string rule_name;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                if(line[0] == '{')
                {
                    part_t part;
                    //parse parts
                    pos = line.find('x');
                    if(std::string::npos != pos)
                    {
                        part.x = std::stoi(line.substr(pos+2));
                    }
                    pos = line.find('m');
                    if(std::string::npos != pos)
                    {
                        part.m = std::stoi(line.substr(pos+2));                        
                    }
                    pos = line.find('a');
                    if(std::string::npos != pos)
                    {
                        part.a = std::stoi(line.substr(pos+2));                        
                    }
                    pos = line.find('s');
                    if(std::string::npos != pos)
                    {
                        part.s = std::stoi(line.substr(pos+2));                        
                    }
                    part.accepted = false;
                    part.rating = part.x + part.m + part.a + part.s;
                    std::cout << "{x=" << part.x << ",m=" << part.m << ",a=" << part.a << ",s=" << part.s << "}" << std::endl;
                    parts.push_back(part);
                }
                else
                {
                    //parse filter rules
                    filter_t filter;
                    pos = line.find('{');
                    rule_name = line.substr(0, pos);
                    line = line.substr(pos+1);

                    std::cout << rule_name << '{';

                    while(line.length() > 1)
                    {
                        next = line.find(',');
                        if(std::string::npos != next)
                        {
                            rule_t rule;
                            rule.id = line[0];
                            rule.op = line[1];
                            rule.val = std::stoi(line.substr(2));
                            pos = line.find(':');
                            rule.go = line.substr(pos+1, next-pos-1);
                            
                            std::cout << rule.id << rule.op << rule.val << ":" << rule.go << ",";

                            filter.rules.push_back(rule);
                        }
                        else
                        {
                            next = line.find('}');
                            filter.def = line.substr(0, next);
                            std::cout << filter.def << '}';
                        }
                        line = line.substr(next+1);

                    }
                    filters[rule_name] = filter;
                    std::cout << std::endl;
                }
            }
            else
            {
                std::cout << std::endl;
            }
        }
        
        std::cout << std::endl;
        file.close();
    }
}


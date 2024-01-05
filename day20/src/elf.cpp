#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

pulse_t pulse_ff(module_t & ff)
{
    pulse_t p = NONE;

    pulse_t pulse = ff.pulses.front().first;
    ff.pulses.erase(ff.pulses.begin());
    
    if(pulse == LOW)
    {
        if(ff.state == OFF)
        {
            ff.state = ON;
            p = HIGH;
        }
        else
        {
            ff.state = OFF;
            p = LOW;
        }
    }
    return p;
}

pulse_t pulse_con(module_t & con)
{
    pulse_t p = HIGH;

    std::pair<pulse_t,std::string> pulse = con.pulses.front();
    con.pulses.erase(con.pulses.begin());

    int count = 0;

    con.inputs[pulse.second] = pulse.first;

    for(auto & [key, val]: con.inputs)
    {
        if(val == HIGH)
        {
            count++;
        }
    }

    if(con.inputs.size() == count)
    {
        p = LOW;
    }

    return p;
}

std::tuple<uint64_t, uint64_t, uint64_t, uint64_t> push_button(std::map<std::string, module_t> & modules)
{
    std::vector<std::string> queue;
    module_t broad = modules["broadcaster"];
    uint64_t pulses_low = 1;
    uint64_t pulses_high = 0;
    uint64_t pulses_low_rx = 0;
    uint64_t pulses_high_rx = 0;
    
    for(auto & t : broad.targets)
    {
        modules[t].pulses.push_back({LOW, "broadcaster"});
        queue.push_back(t);
        pulses_low++;
    }

    uint limit = 0;

    std::string current;
    module_t mod;
    pulse_t p;
    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        mod = modules[current];

        if(mod.type == FF)
        {
            p = pulse_ff(mod);
            modules[current] = mod;
        }
        else if(mod.type == CON)
        {
            p = pulse_con(mod);
            modules[current] = mod;
        }
        else if(mod.type == BROAD)
        {
            p = mod.pulses.front().first;
            mod.pulses.erase(mod.pulses.begin());
            modules[current] = mod;
        }
        else
        {
            std::cout << "UNEXPECTED" << std::endl;
            std::exit(-1);
        }

        if(p != NONE)
        {
            for(auto & t : mod.targets)
            {
                modules[t].pulses.push_back({p, current});
                queue.push_back(t);
                if(p == LOW)
                {
                    pulses_low++;
                    if(current == "vt")
                    {
                        pulses_low_rx++;
                    }
                }
                else
                {
                    pulses_high++;
                    if(current == "vt")
                    {
                        pulses_high_rx++;
                    }
                }
                //std::cout << current << " " << p << " " << t << std::endl;
            }
        }
    }

    return {pulses_low, pulses_high, pulses_low_rx, pulses_high_rx};
}

uint64_t Elves::get_pulses(int cnt)
{
    uint64_t pulses_low = 0;
    uint64_t pulses_high = 0;

    std::map<std::string, module_t> mod_cpy = modules;

    for(int i = 0; i < cnt; i++)
    {
        auto [low, high, rx_l, r_h] = push_button(mod_cpy);
        pulses_low += low;
        pulses_high += high;
        //std::cout << low << "; " << high << std::endl;
    }
    
    //std::cout << "Total: " << pulses_low << "; " << pulses_high << std::endl;


    return pulses_low * pulses_high;
}

uint64_t Elves::get_rx_pulse()
{
    uint64_t presses = 0;
    uint64_t pulses_low = 0;
    uint64_t pulses_high = 0;

    std::map<std::string, module_t> mod_cpy = modules;

    while(pulses_high != 1)
    {
        auto [low, high, rx_l, rx_h] = push_button(mod_cpy);
        pulses_low = rx_l;
        pulses_high = rx_h;
        presses++;
        std::cout << rx_l << "; " << rx_h << std::endl;
    }

    return presses;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            std::string name = "";
            module_t module;
            size_t pos = 0;
            if(line[0] == '%')
            {
                module.type = FF;
                module.state = OFF;
                pos = line.find(" -> ");
                name = line.substr(1, pos - 1);
            }
            else if(line[0] == '&')
            {
                module.type = CON;
                pos = line.find(" -> ");
                name = line.substr(1, pos - 1);
            }
            else if(std::string::npos != line.find("broadcaster"))
            {
                module.type == BROAD;
                name = "broadcaster";
                pos = line.find(" -> ");
            }

            std::stringstream targets(line.substr(pos+3));
            while (std::getline(targets, line, ','))
            {
                std::string target = line.substr(1);
                module.targets.push_back(target);
            }

            modules[name] = module;
        }

        //find all inputs for conjunctions
        for (auto & [n, mod] : modules)
        {
            if(mod.type == CON)
            {
                for (auto & [key, val] : modules)
                {
                    for(auto & t : val.targets)
                    {
                        if(t == n )
                        {
                            modules[n].inputs[key] = LOW;
                        }
                    }
                }
            }
        }
        std::cout << std::endl;
        file.close();
    }
}


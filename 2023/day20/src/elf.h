#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <map>

/* Flip-flop modules (prefix %) are either on or off; they are initially off. If a 
 flip-flop module receives a high pulse, it is ignored and nothing happens. However, 
 if a flip-flop module receives a low pulse, it flips between on and off. If it was 
 off, it turns on and sends a high pulse. If it was on, it turns off and sends a 
 low pulse.*/

/* Conjunction modules (prefix &) remember the type of the most recent pulse received
 from each of their connected input modules; they initially default to remembering a 
 low pulse for each input. When a pulse is received, the conjunction module first 
 updates its memory for that input. Then, if it remembers high pulses for all inputs, 
 it sends a low pulse; otherwise, it sends a high pulse.*/

enum type_t {
    FF=0,
    CON=1,
    BROAD=2
};

enum state_t {
    OFF=0,
    ON=1
};

enum pulse_t {
    LOW=0,
    HIGH=1,
    NONE=2
};

struct module_t {
    type_t type;
    state_t state;
    std::map<std::string, pulse_t> inputs;
    std::vector<std::pair<pulse_t, std::string>> pulses;
    std::vector<std::string> targets;
};

class Elves {
    private:
        std::map<std::string, module_t> modules;

    public:
        Elves(char * file_name);
        uint64_t get_pulses(int cnt);
        uint64_t get_rx_pulse();

};

#endif /* ELF_H */



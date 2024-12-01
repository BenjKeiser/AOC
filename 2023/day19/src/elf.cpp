#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <set>

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
enum val_t {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
    UNKNOWN = 99
};

struct range_t {
    int start;
    int end;

    bool const operator<(const range_t &r) const {
        if(start != r.start)
        {
            return start < r.start;
        }
        return end < r.end;
    }
};

struct ranges_t {
    range_t ranges[4];

    uint64_t get_combs()
    {
        uint64_t combs = 1;
        for(int i = 0; i < 4; i++)
        {
            combs *= (ranges[i].end - ranges[i].start + 1);
        }
        return combs;
    }
};

struct node_t {
    std::string id;
    std::vector<node_t> next;
    ranges_t ranges;
};

struct steps_t {
    val_t id;
    char op;
    int val;
};

struct next_t {
    std::string id;
    std::vector<steps_t> steps;
};

void print_ranges(ranges_t & ranges)
{
    for(int i = 0; i < 4; i++)
    {
        std::cout << "R" << i << ": " << "{" << ranges.ranges[i].start << ", " << ranges.ranges[i].end << "}" << std::endl;

    }
    std::cout << std::endl;
}

val_t char_to_val(char c)
{
    val_t v = UNKNOWN;
    if(c == 'x')
    {
        v = X; 
    }
    else if (c == 'm')
    {
        v = M;
    }
    else if (c == 'a')
    {
        v = A;
    }
    else if (c == 's')
    {
        v = S;
    }
    else
    {
        v = UNKNOWN;
    }
    
    return v;
};

void swap_rule(char * op, int * val)
{
    if(*op == '<')
    {
        *op = '>';
        *val -= 1;
    }
    else
    {
        *op = '<';
        *val += 1;
    }
}

bool apply_rule_to_range(steps_t step, ranges_t & ranges)
{
    bool accept = true;
    
    //std::cout << (int)step.id << step.op << step.val << std::endl;
    //std::cout << ranges.ranges[step.id].start << ", " << ranges.ranges[step.id].end << " -> " << step.val << std::endl;

    if(ranges.ranges[step.id].start <= step.val && ranges.ranges[step.id].end >= step.val)
    {
        // step is within range -> we split at the desired point
        if(step.op == '>')
        {
            ranges.ranges[step.id].start = step.val + 1;
        }
        else
        {
            ranges.ranges[step.id].end = step.val - 1;
        }
    }
    else if(ranges.ranges[step.id].end <= step.val)
    {
        // full range is below step
        if(step.op == '>')
        {
            ranges.ranges[step.id].start = 0;
            ranges.ranges[step.id].end = 0;
            accept = false;
        }
        else
        {
            //nothing
        }
    }
    else if(ranges.ranges[step.id].start >= step.val)
    {
        // full range is above step
        if(step.op == '>')
        {
            //nothing
        }
        else
        {
            ranges.ranges[step.id].start = 0;
            ranges.ranges[step.id].end = 0;
            accept = false;
        }
    }
    else
    {
        std::cout << "Did not expect to get here! " << std::endl;
        std::exit(-1);
    }
    return accept;
}

std::vector<node_t> get_next_nodes(filter_t cf, ranges_t rs)
{
    std::vector<node_t> nodes;
    std::vector<next_t> calc;

    std::vector<steps_t> next_steps;

    for(auto & f : cf.rules)
    {
        next_t current;
        steps_t s;
        
        current.steps.insert(current.steps.end(), next_steps.begin(), next_steps.end());
        current.id = f.go;

        s.op = f.op;
        s.id = char_to_val(f.id);
        s.val = f.val;

        current.steps.push_back(s);

        //we need to store the requirement to pass over this step in order to reach the next step
        swap_rule(&s.op, &s.val);
        next_steps.push_back(s);

        calc.push_back(current);
    }

    //add the default node -> needs to not fulfill any of the previous requirements
    next_t def;
    def.id = cf.def;
    def.steps.insert(def.steps.end(), next_steps.begin(), next_steps.end());
    calc.push_back(def);

    //we collected all the rules, time to do some ranges magic
    for(auto & c : calc)
    {
        node_t current;
        ranges_t ro = rs;
        bool accept = true;

        current.id = c.id;
        for(auto & r : c.steps)
        {
            //apply rule to range
            accept = apply_rule_to_range(r, ro);
            if(!accept)
            {
                std::cout << "Here is an issue" << std::endl;
                std::exit(-1);
            }
        }
        current.ranges = ro;
        nodes.push_back(current);
    }

    return nodes;
}

uint64_t Elves::get_combinations()
{
    uint64_t comb = 0;

    std::vector<node_t> queue;
    filter_t cf;

    //initialize start ranges
    ranges_t ranges;
    ranges.ranges[0] = {1, 4000};
    ranges.ranges[1] = {1, 4000};
    ranges.ranges[2] = {1, 4000};
    ranges.ranges[3] = {1, 4000};

    //initialize start node
    node_t current;
    current.id = "in";
    current.ranges = ranges;

    queue.push_back(current);
    int i = 0;
    while(!queue.empty())
    {
        current = queue.front();
        queue.erase(queue.begin());

        //std::cout << "Visiting Node: " << current.id << std::endl;

        cf = filters[current.id];

        current.next = get_next_nodes(cf, current.ranges);

        //std::cout << "Next Node: " << std::endl;;
        for(auto & n : current.next)
        {
            if(n.id == "A")
            {
                //std::cout << "Adding Range:" << std::endl;
                //print_ranges(n.ranges);
                comb += n.ranges.get_combs();
            }
            else if(n.id == "R")
            {

            }
            else
            {
                //std::cout << n.id << std::endl;
                //print_ranges(n.ranges);
                queue.push_back(n);
            } 
        }
        i++;
        if(i > 5)
        {
            //break;
        }
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


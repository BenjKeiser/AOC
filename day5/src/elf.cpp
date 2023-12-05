#include "elf.h"
#include <numeric>
#include <algorithm>
#include <iostream>
#include <sstream>

//add all the maps for this type
void Elves::add_maps(std::vector<map_t> * maps)
{
    std::string line;
    if (file.is_open()) 
    {
        while(std::getline(file, line))
        {
            if(line.length() > 0)
            {
                std::istringstream s(line);
                map_t map;
                s >> map.nb;
                s >> map.m_start;
                s >> map.m_range;
                maps->push_back(map);
            }
            else
            {
                break;
            }
        }
    }
}

//get mapping value for an input
int64_t Elves::get_map(int64_t nb, std::vector<map_t> * mappings)
{
    int64_t mapv = 0;
    bool found = false;
    // try to find mapping
    for(auto & m : *mappings)
    {
        if(nb >= m.m_start && (nb <= m.m_start + m.m_range))
        {
            mapv = nb - (m.m_start - m.nb);
            found = true;
            break;
        }
    }
    // no mapping found -> it is 1 to 1 mapped
    if(false == found)
    {
        mapv = nb;
    }
    return mapv;
}

//get the closest location by going through all the maps
int64_t Elves::get_closest_location()
{
    int64_t location = INT64_MAX;
    int64_t nb = 0;
    for(auto & seed : seeds)
    {
        nb = get_map(seed, &seed_to_soil);
        nb = get_map(nb, &soil_to_fertilizer);
        nb = get_map(nb, &fertilizer_to_water);
        nb = get_map(nb, &water_to_light);
        nb = get_map(nb, &light_to_temperature);
        nb = get_map(nb, &temperature_to_humidity);
        nb = get_map(nb, &humidity_to_location);
        if( nb < location)
        {
            location = nb;
        }
    }
    return location;
}

//function to sort the maps smallest map first
bool comp_maps(map_t m1, map_t m2) 
{ 
    return (m1.nb < m2.nb); 
} 

//split input ranges according to mapping
std::vector<range_t> split_into_ranges(range_t in, std::vector<map_t> * map)
{
    std::vector<range_t> ranges;
    std::vector<range_t> split_ranges;

    bool split = false;
    
    ranges.push_back(in);
    range_t r;

    while(!ranges.empty())
    {
        r = ranges.back();
        ranges.pop_back();
        split = false;
        for(auto & m : *map)
        {
            if(m.m_start <= r.start)
            {
                //map start is smaller
                if(m.m_start + m.m_range > r.start)
                {
                    //we cover part of the range, split it
                    if(m.m_start + m.m_range >= r.start + r.range)
                    {
                        // we cover the whole range, nothing to do
                    }
                    else
                    {
                        ranges.push_back({r.start, m.m_start + m.m_range - r.start});
                        ranges.push_back({m.m_start + m.m_range, r.range - (m.m_start + m.m_range - r.start)});
                        split = true;
                        break;
                    }
                }
            }
            else
            {
                //map start is bigger
                if(m.m_start < r.start + r.range)
                {
                    //we cover part of the range, split it
                    if(m.m_start + m.m_range >= r.start + r.range)
                    {
                        ranges.push_back({r.start, m.m_start - r.start});
                        ranges.push_back({m.m_start, r.range - (m.m_start - r.start)});
                        split = true;
                        break;
                    }

                }
            }
        }
        if(split != true)
        {
            split_ranges.push_back(r);
        }
    }
    return split_ranges;
}

//map a range
range_t map_range(range_t in, std::vector<map_t> * map)
{
    range_t mr;
    bool found = false;
    for(auto & m : *map)
    {
        if(in.start >= m.m_start && (in.start <= m.m_start + m.m_range))
        {
            mr.start = in.start - (m.m_start - m.nb);
            mr.range = in.range;
            found = true;
            break;
        }
    }
    // no mapping found -> it is 1 to 1 mapped
    if(false == found)
    {
        mr = in;
    }
    return mr;
}

std::vector<range_t> get_ranges_from_maps(std::vector<range_t> * ranges, std::vector<map_t> * maps)
{
    std::vector<range_t> r;
    for(auto & s : *ranges)
    {
        std::vector<range_t>tmp = split_into_ranges(s, maps);
        for(auto & t : tmp)
        {
            r.push_back(map_range(t, maps));
        }
    }
    return r;
}

int64_t Elves::get_closest_location2()
{
    int64_t location = INT64_MAX;
    std::vector<range_t> srange;
    std::vector<range_t> soils;
    std::vector<range_t> fertilizer;
    std::vector<range_t> water;
    std::vector<range_t> light;
    std::vector<range_t> temperature;
    std::vector<range_t> humidity;
    std::vector<range_t> locations;

    // first we sort all the maps
    std::sort(seed_to_soil.begin(), seed_to_soil.end(), comp_maps); 
    std::sort(soil_to_fertilizer.begin(), soil_to_fertilizer.end(), comp_maps); 
    std::sort(fertilizer_to_water.begin(), fertilizer_to_water.end(), comp_maps); 
    std::sort(water_to_light.begin(), water_to_light.end(), comp_maps); 
    std::sort(light_to_temperature.begin(), light_to_temperature.end(), comp_maps); 
    std::sort(temperature_to_humidity.begin(), temperature_to_humidity.end(), comp_maps); 
    std::sort(humidity_to_location.begin(), humidity_to_location.end(), comp_maps); 

    // fill seeds into ranges
    for(int64_t i = 0; i < seeds.size(); i+=2)
    {
        srange.push_back({seeds[i], seeds[i+1]});
    }

    soils = get_ranges_from_maps(&srange, &seed_to_soil);
    fertilizer = get_ranges_from_maps(&soils, &soil_to_fertilizer);
    water = get_ranges_from_maps(&fertilizer, &fertilizer_to_water);
    light = get_ranges_from_maps(&water, &water_to_light);
    temperature = get_ranges_from_maps(&light, &light_to_temperature);
    humidity = get_ranges_from_maps(&temperature, &temperature_to_humidity);
    locations = get_ranges_from_maps(&humidity, &humidity_to_location);

    for(int i = 0; i < locations.size(); i++)
    {
        if(locations[i].start < location)
        {
            location = locations[i].start;
        }
    }
    
    return location;
}

Elves::Elves(char * file_name)
{
    file.open(file_name);
    std::string line;

    int64_t nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(std::string::npos != line.find("seeds:"))
            {
                std::istringstream s(line.substr(6));
                while(s >> nb)
                {
                    seeds.push_back(nb);
                }
            }

            if(std::string::npos != line.find("seed-to-soil"))
            {
                add_maps(&seed_to_soil);
            }

            if(std::string::npos != line.find("soil-to-fertilizer"))
            {
                add_maps(&soil_to_fertilizer);
            }
            
            if(std::string::npos != line.find("fertilizer-to-water"))
            {
                add_maps(&fertilizer_to_water);
            }
            
            if(std::string::npos != line.find("water-to-light"))
            {
                add_maps(&water_to_light);
            }
            
            if(std::string::npos != line.find("light-to-temperature"))
            {
                add_maps(&light_to_temperature);
            }
            
            if(std::string::npos != line.find("temperature-to-humidity"))
            {
                add_maps(&temperature_to_humidity);
            }

            if(std::string::npos != line.find("humidity-to-location"))
            {
                add_maps(&humidity_to_location);
            }
        }
        file.close();
    }
}


#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>
#include <fstream>

struct map_t {
    int64_t nb;
    int64_t m_start;
    int64_t m_range;
};

struct range_t {
    int64_t start;
    int64_t range;
};

class Elves {
    private:
        std::ifstream file;
        std::vector<int64_t> seeds;
        std::vector<map_t> seed_to_soil;
        std::vector<map_t> soil_to_fertilizer;
        std::vector<map_t> fertilizer_to_water;
        std::vector<map_t> water_to_light;
        std::vector<map_t> light_to_temperature;
        std::vector<map_t> temperature_to_humidity;
        std::vector<map_t> humidity_to_location;
    public:
        Elves(char * file_name);
        void add_maps(std::vector<map_t> * maps);
        int64_t get_map(int64_t nb, std::vector<map_t> * mappings);
        int64_t get_closest_location();
        int64_t get_closest_location2();
};

#endif /* ELF_H */



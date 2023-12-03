#ifndef ELF_H
#define ELF_H

#include <vector>
#include <string>
#include <utility>

class Elves {
    private:
        std::vector<std::string> calibration;
        char get_first_digit(std::string calib);
        char get_last_digit(std::string calib);
        void replace_digits(std::string * line);
    public:
        Elves(char * file_name);
        int get_calib_data();
};

#endif /* ELF_H */



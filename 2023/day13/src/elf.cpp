#include "elf.h"
#include <numeric>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

struct smudge_t {
    int i;
    int k;
    int z;
};

void print_pattern(std::vector<std::string> * pattern)
{
    for(auto & p : *pattern)
    {
        //std::cout << p << std::endl;
    }
}

bool get_vertical_smudge(std::vector<std::string> pattern, int smudge_max, smudge_t * smudge, std::vector<int> mirror_lines)
{
    bool mirror = false;
    bool skip = false;
    int smudge_count = 0;

    for(int i = 0; i < pattern[0].length()-1; i++)
    {
        mirror = true;
        smudge_count = 0;
        skip=false;
        for(auto & s : mirror_lines)
        {
            if(s == i)
            {
                mirror=false;
                skip=true;
                break;
            }
        }
        if(skip)
        {
            continue;
        }
        for(int k = 0; k <= std::min(i, (int)(pattern[0].length()-i-2)); k++)
        {
            for(int z = 0; z < pattern.size(); z++)
            {
                if(pattern[z][i-k] != pattern[z][i+k+1])
                {
                    if(smudge_count < smudge_max)
                    {
                        *smudge = {i, k, z};
                        smudge_count++;
                    }
                    else
                    {
                        smudge_count=0;
                        mirror = false;
                        break;
                    }
                }
            }
        }
        if(mirror && (smudge_count <= smudge_max))
        {
            if(smudge_max != 0)
            {
                if(smudge_count > 0)
                {
                    break;
                }
            }
        }
    }

    return mirror;
}

bool get_horizontal_smudge(std::vector<std::string> pattern, int smudge_max, smudge_t * smudge, std::vector<int> mirror_lines)
{
    bool mirror = false;
    bool skip = false;
    int smudge_count = 0;

    for(int i = 0; i < pattern.size()-1; i++)
    {
        mirror = true;
        smudge_count = 0;
        skip=false;
        for(auto & s : mirror_lines)
        {
            if(s == i)
            {
                skip=true;
                mirror=false;
                break;
            }
        }
        if(skip)
        {
            continue;
        }

        for(int k = 0; k <= std::min(i, (int)(pattern.size()-i-2)); k++)
        {
            for(int z = 0; z < pattern[0].length(); z++)
            {
                if(pattern[i-k][z] != pattern[i+k+1][z])
                {
                    if(smudge_count < smudge_max)
                    {
                        *smudge = {i, k, z};
                        smudge_count++;
                    }
                    else
                    {
                        smudge_count=0;
                        mirror = false;
                        break;
                    }
                }
            }
        }
        if(mirror && (smudge_count <= smudge_max))
        {
            if(smudge_max != 0)
            {
                if(smudge_count > 0)
                {
                    break;
                }
            }
        }
    }

    return mirror;
}

int get_vertical_mirror(std::vector<std::string> pattern, std::vector<int> * line)
{
    int mirrors = 0;

    bool mirror = false;

    std::vector<int> mirror_line = *line;

    line->clear();
    for(int i = 0; i < pattern[0].length()-1; i++)
    {
        mirror = true;
        for(auto & ml : mirror_line)
        {
            if(i == ml)
            {
                mirror = false;
                break;
            }
        }
        for(int k = 0; k <= std::min(i, (int)(pattern[0].length()-i-2)); k++)
        {
            for(int z = 0; z < pattern.size(); z++)
            {
                if(pattern[z][i-k] != pattern[z][i+k+1])
                {
                    mirror = false;
                    break;
                }
            }
        }
        if(mirror)
        {
            //std::cout << "Vertical Mirror: " << i+1 << std::endl;
            mirrors += i + 1;
            line->push_back(i);
        }
    }

    return mirrors;
}

int get_horizontal_mirror(std::vector<std::string> pattern, std::vector<int> * line)
{
    int mirrors = 0;

    bool mirror = false;
    
    smudge_t smudge;

    std::vector<int> mirror_line = *line;

    line->clear();

    for(int i = 0; i < pattern.size()-1; i++)
    {
        mirror = true;
        for(auto & ml : mirror_line)
        {
            if(i == ml)
            {
                mirror = false;
                break;
            }
        }

        for(int k = 0; k <= std::min(i, (int)(pattern.size()-i-2)); k++)
        {
            if(pattern[i-k] != pattern[i+k+1])
            {
                mirror = false;
                break;
            }
        }
        if(mirror)
        {
            //std::cout << "Horizontal Mirror: " << i+1 << std::endl;
            mirrors += i + 1;
            line->push_back(i);
        }
    }

    return mirrors;
}

int Elves::get_mirrors(int smudge_count)
{
    int mirrors = 0;
    int smudge_mirrors = 0;
    smudge_t vsmudge;
    smudge_t hsmudge;
    bool found_smudge = false;
    std::vector<int> horizontal_lines;
    std::vector<int> vertical_lines;
    for(auto & p : patterns)
    {
        //std::cout << "================" << std::endl;
        std::vector<std::string> pattern = p;
        vertical_lines.clear();
        horizontal_lines.clear();
        print_pattern(&pattern);   

        mirrors += get_vertical_mirror(pattern, &vertical_lines);
        
        mirrors += 100 * get_horizontal_mirror(pattern, &horizontal_lines);

        if(smudge_count > 0)
        {
            found_smudge = get_vertical_smudge(pattern, smudge_count, &vsmudge, vertical_lines);
            if(found_smudge)
            {
                //std::cout << "Smudge V: " << vsmudge.i << " " << vsmudge.k << " " << vsmudge.z << std::endl;
                #if 0
                found_smudge = get_horizontal_smudge(pattern, smudge_count, &hsmudge, horizontal_lines);
                if(found_smudge)
                {
                    //std::cout << "Smudge H: " << hsmudge.i << " " << hsmudge.k << " " << hsmudge.z << std::endl;
                    //std::cout << "Well shit..." << std::endl;
                    print_pattern(&pattern);
                    break;
                }
                #endif
                pattern[vsmudge.z][vsmudge.i-vsmudge.k] = pattern[vsmudge.z][vsmudge.i+vsmudge.k+1];
            }
            else
            {
                found_smudge = get_horizontal_smudge(pattern, smudge_count, &hsmudge, horizontal_lines);
                if(found_smudge)
                {
                    //std::cout << "Smudge H: " << hsmudge.i << " " << hsmudge.k << " " << hsmudge.z << std::endl;
                    pattern[hsmudge.i-hsmudge.k][hsmudge.z] = pattern[hsmudge.i+hsmudge.k+1][hsmudge.z];
                }
                else
                {
                    //std::cout << "Well this is unexpected..." << std::endl;
                    print_pattern(&pattern);
                    break;
                }
            }

            print_pattern(&pattern); 
            
            smudge_mirrors += get_vertical_mirror(pattern, &vertical_lines);
        
            smudge_mirrors += 100 * get_horizontal_mirror(pattern, &horizontal_lines);
            
        }     
    }

    if(smudge_count > 0)
    {
        mirrors = smudge_mirrors;
    }

    return mirrors;
}

Elves::Elves(char * file_name)
{
    std::ifstream file(file_name);
    std::string line;

    std::vector<std::string> pattern;
    int nb = 0;
    // parse the file
    if (file.is_open()) 
    {
        while (std::getline(file, line)) 
        {
            if(line.length() > 0)
            {
                pattern.push_back(line);
            }
            else
            {
                patterns.push_back(pattern);
                pattern.clear();
            }
        }
        patterns.push_back(pattern);
        pattern.clear();
        
        file.close();
    }
}


/*
 * For part 2, redirect stdout to a file and then search for the cue in that
 * file.
 */

#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * Return index of maximum value in the given array. If multiple positions have
 * same values, smaller index is returned.
 */
static int find_max_char(int* char_occs)
{
    int max_so_far = 0;
    int max_idx    = 0;

    for (int i = 25; i >= 0; --i)
    {
        if (char_occs[i] >= max_so_far)
        {
            max_so_far = char_occs[i];
            max_idx    = i;
        }
    }

    return max_idx;
}

static int real_num_sector_id(char* line)
{
    int char_occs[26] = { 0 };

    int  i = 0;
    char c = line[i];
    int  sector_id = 0;

    for (;;)
    {
        if (isdigit(c))
        {
            // read sector number
            while (isdigit(c))
            {
                sector_id *= 10;
                sector_id += c - '0';
                c = line[++i];
            }

            // read checksum
            assert(c == '[');
            c = line[++i];

            bool checksum_correct = true;
            while (c != ']')
            {
                int max_idx = find_max_char(char_occs);
                char_occs[max_idx] = 0; // to avoid getting same char back
                checksum_correct &= max_idx == c - 'a';
                c = line[++i];
            }

            return checksum_correct ? sector_id : 0;
        }
        else
        {
            if (c != '-')
                char_occs[c - 'a']++;
            c = line[++i];
        }
    }

    assert(false);
}

/**
 * For the part 2.
 */
void decrypt(char* line)
{
    // skip until sector id
    int i = 0;
    char c = line[i];
    while (!isdigit(c))
        c = line[++i];

    int sector_id = 0;
    while (isdigit(c))
    {
        sector_id *= 10;
        sector_id += c - '0';
        c = line[++i];
    }

    assert(c == '[');

    // shift the string
    i = 0;
    c = line[i];
    while (!isdigit(c))
    {
        if (c != '-')
            line[i] = ((c - 'a' + sector_id) % 26) + 'a';
        c = line[++i];
    }

    printf("decrypted line: %s\n", line);
}

int main()
{
    // let's not allocate on stack :O
    char* line = malloc(1000);

    int sum = 0;
    while (scanf("%s\n", line) == 1)
    {
        sum += real_num_sector_id(line);
        decrypt(line); // this modifies 'line'!
    }

    printf("real room sector id sum: %d\n", sum);

    return 0;
}

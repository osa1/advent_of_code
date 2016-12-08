#include "common.h"

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

bool check_abba(char* str, int len)
{
    printf("check_abba(\"%.*s\");\n", len, str);

    for (int i = 0; i < len - 3; ++i)
        if (str[i] == str[i + 3] && str[i + 1] == str[i + 2]
                && str[i] != str[i + 1])
            return true;

    return false;
}

typedef struct three_letter_
{
    char l1;
    char l2;
    // l3 is always same as l1
} three_letter;

static inline
bool three_letter_eq(three_letter l1, three_letter l2)
{
    return l1.l1 == l2.l1 && l1.l2 == l2.l2;
}

int fill_abas(char* str, int len, three_letter* abas, int n_abas)
{
    int ret = 0;

    for (int i = 0; i < len - 2; ++i)
    {
        if (str[i] == str[i + 2] && str[i] != str[i + 1])
        {
            (abas + n_abas + ret)->l1 = str[i    ];
            (abas + n_abas + ret)->l2 = str[i + 1];
            ++ret;
        }
    }

    return ret;
}

int main()
{
    buffer buf;
    buffer_init(&buf, 1000);
    read_stdin(&buf);

    char* input     = (char*)buf.buf;
    int   input_len = buf.len;

    int   ret       = 0; // part 1
    int   ret2      = 0; // part 2

    for (int i = 0; i < input_len;)
    {
        // Scan one line
        int p = i;
        bool found_abba = false;
        bool found_invalid_abba = false;

        // For part 2, we collect ABA and BAB codes
        three_letter abas[1000];
        int num_abas = 0;
        three_letter babs[1000];
        int num_babs = 0;

        for (;;)
        {
            if (input[i] == '[')
            {
                if (check_abba(input + p, i - p))
                    found_abba = true;
                num_abas += fill_abas(input + p, i - p, abas, num_abas);
                ++i;
                p = i;
            }
            else if (input[i] == ']')
            {
                if (check_abba(input + p, i - p))
                    found_invalid_abba = true;
                num_babs += fill_abas(input + p, i - p, babs, num_babs);
                ++i;
                p = i;
            }
            else if (input[i] == '\n')
            {
                if (check_abba(input + p, i - p))
                    found_abba = true;
                num_abas += fill_abas(input + p, i - p, abas, num_abas);
                break;
            }
            else
                ++i;
        }

        assert(input[i] == '\n');
        ++i;

        if (found_abba && !found_invalid_abba)
            ++ret;

        printf("abas:\n");
        for (int i = 0; i < num_abas; ++i)
            printf(" - %c%c%c\n", abas[i].l1, abas[i].l2, abas[i].l1);
        printf("babs:\n");
        for (int i = 0; i < num_babs; ++i)
            printf(" - %c%c%c\n", babs[i].l1, babs[i].l2, babs[i].l1);

        // For a BAB, search for its corrsponding ABA
        bool found_aba = false;
        for (int bab_idx = 0; bab_idx < num_babs; ++bab_idx)
        {
            three_letter aba = { .l1 = babs[bab_idx].l2, .l2 = babs[bab_idx].l1 };
            for (int aba_idx = 0; aba_idx < num_abas; ++aba_idx)
                if (three_letter_eq(aba, abas[aba_idx]))
                {
                    found_aba = true;
                    break;
                }

            // Maybe use a goto?
            if (found_aba)
                break;
        }

        if (found_aba)
            ++ret2;
    }

    printf("part 1: %d\n", ret);
    printf("part 2: %d\n", ret2);

    buffer_free(&buf);
    return 0;
}

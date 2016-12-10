#include <assert.h>
#include <ctype.h>
#include <stdlib.h>
#include <stdio.h>

#include "common.h"

static
long expanded_len(char* input)
{
    // printf("expanded_len(\"%s\");\n", input);

    int  i   = 0;
    long ret = 0;

    while (input[i] != '\0' && input[i] != '\n')
    {
        if (input[i] == '(')
        {
            ++i;

            int n_chars = 0;
            while (isdigit(input[i]))
            {
                n_chars *= 10;
                n_chars += input[i] - '0';
                ++i;
            }

            assert(input[i] == 'x');
            ++i;

            int repeat = 0;
            while (isdigit(input[i]))
            {
                repeat *= 10;
                repeat += input[i] - '0';
                ++i;
            }

            assert(input[i] == ')');
            ++i;

            // prepare string to decompress
            char char_bak = input[i + n_chars];
            input[i + n_chars] = '\0';
            // decompress substring and note the size
            long substr_len = expanded_len(input + i) * repeat;
            // restore original string
            input[i + n_chars] = char_bak;
            // continue scanning the string
            long rest_len = expanded_len(input + i + n_chars);

            return ret + substr_len + rest_len;
        }
        else
        {
            ++i;
            ++ret;
        }
    }

    return ret;
}

int main()
{
    buffer input_buf;
    buffer_init(&input_buf, 1000);
    read_stdin(&input_buf);

    printf("%ld\n", expanded_len((char*)input_buf.buf));

    buffer_free(&input_buf);
    return 0;
}

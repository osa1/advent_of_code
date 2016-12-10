#include <assert.h>
#include <ctype.h>
#include <stdlib.h>
#include <stdio.h>

#include "common.h"

int main()
{

    buffer input_buf;
    buffer_init(&input_buf, 1000);
    read_stdin(&input_buf);

    char* input = (char*)input_buf.buf;

    buffer output_buf;
    buffer_init(&output_buf, input_buf.cap * 2);

    for (int i = 0; i < input_buf.len - 1; ++i)
    {
        if (input[i] == '(')
        {
            ++i;

            // parse ('chars' x 'repeat')
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

            // copy the string to duplicate
            char* str = malloc(n_chars);
            for (int n = 0; n < n_chars; ++n)
                str[n] = input[i + n];

            // write str to output buffer n times
            for (int n = 0; n < repeat; ++n)
                buffer_push_str(&output_buf, (uint8_t*)str, n_chars);

            free(str);

            // skip copied string and continue
            i += n_chars - 1;
        }
        else
        {
            buffer_push(&output_buf, input[i]);
        }
    }

    printf("len: %d\n", output_buf.len);
    // printf("str: \"%s\"\n", output_buf.buf);

    buffer_free(&output_buf);
    buffer_free(&input_buf);
    return 0;
}

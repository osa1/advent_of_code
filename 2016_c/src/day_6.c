#include "common.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef bool(*cmp_fn)(int, int);


char max_char(int* tbl, cmp_fn cmp)
{
    int max_idx = 0;
    int max_val = tbl[0];

    for (int i = 1; i < 26; ++i)
    {
        // TODO: What about ties?
        if (tbl[i] && cmp(tbl[i], max_val))
        {
            max_val = tbl[i];
            max_idx = i;
        }
    }

    return (char)max_idx + 'a';
}

bool int_gt(int i1, int i2)
{
    return i1 > i2;
}

bool int_lt(int i1, int i2)
{
    return i1 < i2;
}

int main(int argc, char** argv)
{
    cmp_fn cmp = int_gt;
    // for part 2
    if (argc != 1)
        cmp = int_lt;

    buffer buf;
    buffer_init(&buf, 1000);
    read_stdin(&buf);

    int num_chars = 0;
    while (buf.buf[num_chars] != '\n')
        ++num_chars;

    printf("number of chars: %d\n", num_chars);

    char* msg = malloc(num_chars + 1);
    msg[num_chars] = '\0';

    for (int char_idx = 0; char_idx < num_chars; ++char_idx)
    {
        int tbl[26] = { 0 };
        for (int i = 0; i < buf.len - num_chars; i += num_chars + 1)
            ++tbl[buf.buf[i + char_idx] - 'a'];

        msg[char_idx] = max_char(tbl, cmp);
    }

    printf("message: \"%s\"\n", msg);

    free(msg);
    buffer_free(&buf);
    return 0;
}

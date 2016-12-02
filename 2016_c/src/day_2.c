#include "common.h"

#include <assert.h>
#include <stdio.h>

static char keypad[9] =
    { '1', '2', '3',
      '4', '5', '6',
      '7', '8', '9' };

static char key(int x, int y)
{
    assert(x >= 0 && x <= 2);
    assert(y >= 0 && y <= 2);
    return keypad[y * 3 + x];
}

int main()
{
    int ret = 0;

    buffer buf;
    buffer_init(&buf, 1000);
    read_stdin(&buf);

    // Initially we press "5"
    int cur_x = 1;
    int cur_y = 1;

    for (int i = 0; i < buf.len; ++i)
    {
        char c = (char)buf.buf[i];

        if (c == 'U')
        {
            if (cur_y > 0)
                --cur_y;
        }
        else if (c == 'D')
        {
            if (cur_y < 2)
                ++cur_y;
        }
        else if (c == 'L')
        {
            if (cur_x > 0)
                --cur_x;
        }
        else if (c == 'R')
        {
            if (cur_x < 2)
                ++cur_x;
        }
        else if (c == '\n')
        {
            printf("%c", key(cur_x, cur_y));
        }
        else
        {
            fprintf(stderr, "Invalid direction: \"%c\"\n", c);
            ret = 1;
            goto exit;
        }
    }

    printf("\n");

exit:
    buffer_free(&buf);
    return ret;
}

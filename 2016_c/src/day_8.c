#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "common.h"

#define WIDTH  50
#define HEIGHT 6

static inline
void set(char* screen, int x, int y)
{
    assert(x < WIDTH);
    assert(y < HEIGHT);
    screen[y * WIDTH + x] = '*';
}

static inline
char get(char* screen, int x, int y)
{
    assert(x < WIDTH);
    assert(y < HEIGHT);
    return screen[y * WIDTH + x];
}

typedef void(*rotate_fn)(char*, int, int);

static
void rotate_col(char* screen, int col, int by)
{
    char col_data[HEIGHT];
    for (int i = 0; i < HEIGHT; ++i)
        col_data[i] = get(screen, col, i);

    for (int i = 0; i < HEIGHT; ++i)
        screen[i * WIDTH + col] = col_data[mod(i - by, HEIGHT)];
}

static
void rotate_row(char* screen, int row, int by)
{
    char row_data[WIDTH];
    for (int i = 0; i < WIDTH; ++i)
        row_data[i] = get(screen, i, row);

    for (int i = 0; i < WIDTH; ++i)
        screen[row * WIDTH + i] = row_data[mod(i - by, WIDTH)];
}

int main()
{
    buffer input_buf;
    buffer_init(&input_buf, 1000);
    read_stdin(&input_buf);

    char screen[WIDTH * HEIGHT];
    memset(screen, '.', WIDTH * HEIGHT);

    char* line = (char*)input_buf.buf;
    while (*line)
    {
        if (line[1] == 'e')
        {
            // rect <int>x<int>

            int width = 0;
            int i = 5; // first char of width
            while (isdigit(line[i]))
            {
                width *= 10;
                width += line[i] - '0';
                ++i;
            }

            assert(line[i] == 'x');
            ++i;

            int height = 0;
            assert(isdigit(line[i]));
            while (isdigit(line[i]))
            {
                height *= 10;
                height += line[i] - '0';
                ++i;
            }

            assert(line[i] == '\n');

            for (int x = 0; x < width; ++x)
                for (int y = 0; y < height; ++y)
                    set(screen, x, y);

            line += i + 1;
        }
        else if (line[1] == 'o')
        {
            // rotate <column|row> <x/y>=<int> by <int>

            rotate_fn rot;

            int col_row_pos;
            int col_row = 0;

            if (line[7] == 'c')
            {
                rot = rotate_col;
                // rotate column x=
                col_row_pos = 16;
            }
            else if (line[7] == 'r')
            {
                rot = rotate_row;
                // rotate row y=
                col_row_pos = 13;
            }
            else
            {
                fprintf(stderr, "[0] Can't parse input: \"%s\"\n", line);
                exit(1);
            }

            while (isdigit(line[col_row_pos]))
            {
                col_row *= 10;
                col_row += line[col_row_pos] - '0';
                ++col_row_pos;
            }

            assert(line[col_row_pos] == ' ');

            int by_idx = col_row_pos + 4;
            int by = 0;
            while (isdigit(line[by_idx]))
            {
                by *= 10;
                by += line[by_idx] - '0';
                ++by_idx;
            }

            rot(screen, col_row, by);

            line += by_idx + 1;
        }
        else
        {
            fprintf(stderr, "[1] Can't parse input: \"%s\"\n", line);
            exit(1);
        }
    }

    int ret = 0;
    for (int i = 0; i < WIDTH * HEIGHT; ++i)
    {
        if (screen[i] == '*')
            ++ret;
    }

    for (int row = 0; row < HEIGHT; ++row)
    {
        for (int col = 0; col < WIDTH; ++col)
            printf("%c ", get(screen, col, row));
        printf("\n");
    }

    printf("%d\n", ret);

    buffer_free(&input_buf);
    return 0;
}

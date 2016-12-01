#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "point_map.h"

// How many bytes to read in a fread(stdin, ...)
#define READ_CHUNK_SIZE 1024

typedef struct buffer_
{
    uint8_t* buf;
    int len;
    int cap;
} buffer;

void buffer_init(buffer* buf, int cap)
{
    assert(cap > 0);
    buf->buf = malloc(cap);
    assert(buf->buf);
    buf->len = 0;
    buf->cap = cap;
}

void buffer_free(buffer* buf)
{
    free(buf->buf);
}

void buffer_reserve(buffer* buf, int amt)
{
    int new_cap = buf->cap;
    while (new_cap - buf->len < amt)
        new_cap *= 2;

    if (new_cap != buf->cap)
    {
        buf->buf = realloc(buf->buf, new_cap);
        buf->cap = new_cap;
    }
}

void read_stdin(buffer* buf)
{
    for (;;)
    {
        buffer_reserve(buf, READ_CHUNK_SIZE);
        int bytes_read = fread((void*)(buf->buf + buf->len), 1, READ_CHUNK_SIZE, stdin);
        buf->len += bytes_read;
        if (bytes_read < READ_CHUNK_SIZE)
            break;
    }
}

typedef enum { N, E, S, W } DIRS;

int mod(int i, int m)
{
    int ret = i % m;
    if (ret < 0)
        ret += m;
    return ret;
}

int main()
{
    buffer buf;
    buffer_init(&buf, 1000);
    read_stdin(&buf);
    --buf.len; // trailing newline

    int ret = 0;

    // Current direction
    DIRS dir = N;

    point_map*  visiteds   = point_map_new();
    bool        hq_visited = false;
    point       hq_pos;

    point current_pos = { .x = 0, .y = 0 };

    int i = 0;
    while (i < buf.len)
    {
        if (i != 0)
            i += 2; // skip ", "

        char c = (char)buf.buf[i];
        if (c == 'R')
            dir = mod(dir + 1, 4);
        else if (c == 'L')
            dir = mod(dir - 1, 4);
        else
        {
            fprintf(stderr, "Invalid turn: %c\n", c);
            ret = 1;
            goto exit;
        }

        ++i;
        c = (char)buf.buf[i];

        int num = 0;
        while (c >= '0' && c <= '9')
        {
            num *= 10;
            num += c - '0';
            ++i;
            c = (char)buf.buf[i];
        }

        if (!hq_visited)
        {
            void* ret;
            // this is inefficient. good thing advent of code doesn't need
            // efficient solutions
            for (int step = 0; step < num; ++step)
            {
                if (dir == N)
                    ++current_pos.y;
                else if (dir == E)
                    ++current_pos.x;
                else if (dir == S)
                    --current_pos.y;
                else if (dir == W)
                    --current_pos.x;
                else
                    assert(false);

                if (point_map_lookup(visiteds, current_pos, &ret))
                {
                    hq_visited = true;
                    hq_pos     = current_pos;
                    // take rest of the steps quickly and stop the loop
                    if (dir == N)
                        current_pos.y += num - step - 1;
                    else if (dir == E)
                        current_pos.x += num - step - 1;
                    else if (dir == S)
                        current_pos.y -= num - step - 1;
                    else if (dir == W)
                        current_pos.x -= num - step - 1;
                    break;
                }
                else
                {
                    point_map_insert(visiteds, current_pos, NULL);
                }
            }
        }
        else
        {
            // fast path
            if (dir == N)
                current_pos.y += num;
            else if (dir == E)
                current_pos.x += num;
            else if (dir == S)
                current_pos.y -= num;
            else if (dir == W)
                current_pos.x -= num;
        }
    }

    printf("final pos: %d, %d\n", current_pos.x, current_pos.y);

    printf("distance from (0, 0): %d\n", abs(current_pos.x) + abs(current_pos.y));
    if (hq_visited)
    {
        printf("distance from hq (%d, %d): %d\n",
               hq_pos.x, hq_pos.y,
               abs(current_pos.x - hq_pos.x) + abs(current_pos.y - hq_pos.y));
        printf("distance from origin (this is the question): %d, %d (%d)\n",
               hq_pos.x, hq_pos.y, abs(hq_pos.x) + abs(hq_pos.y));
    }

exit:
    point_map_free(visiteds);
    buffer_free(&buf);
    return ret;
}

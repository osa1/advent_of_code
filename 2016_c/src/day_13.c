#include "common.h"
#include "queue.h"
#include "set.h"

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static inline
bool is_open(unsigned int x, unsigned int y, unsigned int input)
{
    return __builtin_popcount(x*x + 3*x + 2*x*y + y + y*y + input) % 2 == 0;
}

static const int INPUT = 1364;

typedef struct
{
    int    x;
    int    y;

    // TODO: Ideally this would be a linked list, this is too costly to clone
    vector prevs;
} pos;

int pos_cmp(void* p1, void* p2)
{
    int x1 = ((pos*)p1)->x;
    int x2 = ((pos*)p2)->x;
    int y1 = ((pos*)p1)->y;
    int y2 = ((pos*)p2)->y;

    if (x1 == x2)
    {
        if (y1 == y2)
            return 0;
        else if (y1 > y1)
            return 1;
        return -1;
    }
    else if (x1 > x2)
        return 1;
    return -1;
}

int main()
{
    // printf("sizeof(unsigned int) = %ld\n", sizeof(unsigned int));
    // printf("sizeof(void*)        = %ld\n", sizeof(void*));

    queue q;
    queue_init(&q, 100);

    set visited;
    set_init(&visited, 100);

    pos* cur_pos = malloc(sizeof(pos));
    cur_pos->x = 1;
    cur_pos->y = 1;
    vector_init(&cur_pos->prevs, 1);

    while (!(cur_pos->x == 31 && cur_pos->y == 39))
    {
        set_insert(&visited, cur_pos, pos_cmp);
        unsigned int x = cur_pos->x;
        unsigned int y = cur_pos->y;

        printf("considering position %d %d\n", x, y);

        pos p1 = { .x = x - 1, .y = y     };
        pos p2 = { .x = x,     .y = y - 1 };
        pos p3 = { .x = x + 1, .y = y     };
        pos p4 = { .x = x,     .y = y + 1 };
        pos ps[4] = { p1, p2, p3, p4 };

        for (int i = 0; i < 4; ++i)
        {
            pos p = ps[i];
            printf("considering %d %d\n", p.x, p.y);
            if (p.x >= 0 && p.y >= 0 && is_open(p.x, p.y, INPUT) && !set_member(&visited, &p, pos_cmp))
            {
                // actually allocate the pos
                pos* p_alloc = malloc(sizeof(pos));
                p_alloc->x = p.x;
                p_alloc->y = p.y;
                vector_clone(&p_alloc->prevs, &cur_pos->prevs);
                vector_push(&p_alloc->prevs, (void*)(((uint64_t)x << 32) + (uint64_t)y));

                set_insert(&visited, p_alloc, pos_cmp);
                queue_push(&q, p_alloc);
            }
        }

        if (q.len == 0)
        {
            printf("No more locations to visit!\n");
            printf("%d locations were visited.\n", visited.len);
            exit(1);
        }

        int q_len = q.len;
        cur_pos = (pos*)queue_pop(&q);
        assert(q.len == q_len - 1);
    }

    printf("%d\n", cur_pos->prevs.len);

    // TODO: Clean queue and set contents

    set_free(&visited);
    queue_free(&q);
    return 0;
}

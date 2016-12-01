#include "../src/point_map.h"

#include <assert.h>

int main()
{
    point_map* map = point_map_new();;

    void* ret;
    point p = { .x = 1, .y = 1 };
    assert(point_map_lookup(map, p, &ret) == false);
    point_map_insert(map, p, (void*)1);
    assert(point_map_lookup(map, p, &ret) == true);
    assert(ret == (void*)1);
    point_map_insert(map, p, (void*)2);
    assert(point_map_lookup(map, p, &ret) == true);
    assert(ret == (void*)2);

    for (int x = 0; x < 100; ++x)
    {
        for (int y = 0; y < 100; ++y)
        {
            // printf("inserting %d, %d\n", x, y);
            p.x = x;
            p.y = y;
            point_map_insert(map, p, (void*)(((uint64_t)x << 32) + (uint64_t)y));
        }
    }

    for (int x = 0; x < 100; ++x)
    {
        for (int y = 0; y < 100; ++y)
        {
            // printf("lookup %d, %d\n", x, y);
            void* ret;
            p.x = x;
            p.y = y;
            assert(point_map_lookup(map, p, &ret));
            assert((uint64_t)ret == ((uint64_t)x << 32) + (uint64_t)y);
        }
    }

    point_map_free(map);
    return 0;
}

#include "point_map.h"

#include <assert.h>
#include <stdlib.h>
#include <string.h>

// It's important that EMPTY is 0 here because we memset() the buffer with 0s
// on initialization
typedef enum { EMPTY = 0, USED, TOMBSTONE } entry_type;

typedef struct __attribute__((packed)) entry_
{
    point p;
    void* val;
    entry_type ty;
} entry;

typedef struct point_map_
{
    entry* entries;
    int len;
    int cap;
} point_map;

static inline uint64_t point_hash(point p)
{
    return (53 + (uint64_t)p.y) * 53 + (uint64_t)p.x;
}

point_map* point_map_new()
{
    point_map* map = malloc(sizeof(point_map));

    map->entries = malloc(sizeof(entry) * 1000);
    memset(map->entries, 0, sizeof(entry) * 1000);
    map->len     = 0;
    map->cap     = 1000;

    return map;
}

void point_map_free(point_map* map)
{
    free(map->entries);
    free(map);
    // TODO we should provide a way to map on values to be able to properly free
}

static void point_map_resize(point_map* map)
{
    int         new_cap     = map->cap * 2;
    entry*      new_entries = malloc(sizeof(entry) * new_cap);
    point_map   new_map     = { .entries = new_entries, .len = 0, .cap = new_cap };

    // FIXME inefficient
    for (int i = 0; i < map->cap; ++i)
    {
        entry* ent = &map->entries[i];
        if (ent->ty == USED)
            point_map_insert(&new_map, ent->p, ent->val);
    }

    free(map->entries);
    map->entries = new_entries;
    map->cap = new_cap;
    assert(map->len == new_map.len);
}

void point_map_insert(point_map* map, point p, void* val)
{
    if (map->cap * 3 <= map->len * 4)
            // 3/4 <= len/cap without floating point mess
        point_map_resize(map);

    uint64_t hash = point_hash(p);
    uint64_t idx  = hash % map->cap;
    entry*   ent  = &map->entries[idx];

    // TODO: we should probably have a distance limit here
    while (ent->ty == USED && idx < map->cap)
    {
        if (ent->p.x == p.x && ent->p.y == p.y)
        {
            ent->val = val;
            return;
        }

        ++idx;
        ent = &map->entries[idx];
    }

    if (idx == map->cap)
    {
        point_map_resize(map);
        point_map_insert(map, p, val);
    }
    else
    {
        map->entries[idx].p = p;
        map->entries[idx].val = val;
        map->entries[idx].ty = USED;
    }
}

bool point_map_lookup(point_map* map, point p, void** ret)
{
    uint64_t hash = point_hash(p);
    uint64_t idx  = hash % map->cap;

    while (idx < map->cap)
    {
        entry* ent = &map->entries[idx];

        if (ent->ty == EMPTY)
            return false;
        if (ent->ty == TOMBSTONE)
        {
            ++idx;
            continue;
        }

        assert(ent->ty == USED);
        if (ent->p.x == p.x && ent->p.y == p.y)
        {
            *ret = ent->val;
            return true;
        }
        else
            ++idx;
    }

    return false;
}

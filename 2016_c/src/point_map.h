#ifndef __POINT_MAP_H
#define __POINT_MAP_H

#include <stdbool.h>
#include <stdint.h>

typedef struct point_map_ point_map;

typedef struct point_
{
    int32_t x;
    int32_t y;
} point;

point_map* point_map_new();
void point_map_free(point_map*);
void point_map_insert(point_map*, point, void*);
bool point_map_lookup(point_map*, point, void**);

#endif

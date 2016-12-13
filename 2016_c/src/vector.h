#ifndef __VECTOR_H
#define __VECTOR_H

/** An extensible vector for pointer-sized things. */
typedef struct
{
    void** buf;
    int    len;
    int    cap;
} vector;

void vector_init(vector*, int cap);
void vector_free(vector*);
void vector_push(vector*, void*);

/** Check len before. */
void* vector_pop(vector*);

/**
 * Insert an element at given index. Elements at indices >= i are shifted
 * forward. 'i' can be at most 'len'.
 */
void vector_insert(vector* vec, void* e, int i);

/**
 * Search through the vector using binary saerch, return given element's index
 * when found. Otherwise return -1.
 *
 * It's caller's responsibility to make sure the vector is sorted.
 */
int vector_bin_search(vector* vec, void* e);

/**
 * Insert an element to an already sorted vector. New vector will be sorted.
 */
void vector_insert_sorted(vector* vec, void* e);

void vector_print(vector*);

#endif

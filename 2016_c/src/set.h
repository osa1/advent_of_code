#ifndef __SET_H
#define __SET_H

#include "vector.h"

#include <stdbool.h>

/** A set of pointer-sized things with trivial equality. */
typedef vector set;

// TODO: cmp_fn should be recorded in 'set' struct

static inline
void set_init(set* s, int cap)
{
    vector_init(s, cap);
}

static inline
void set_free(set* s)
{
    vector_free(s);
}

static inline
void set_insert(set* s, void* e, cmp_fn cmp)
{
    vector_insert_sorted_by(s, e, cmp);
}

static inline
bool set_member(set* s, void* e, cmp_fn cmp)
{
    return vector_bin_search_by(s, e, cmp) != -1;
}

#endif

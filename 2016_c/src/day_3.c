// Um... this is getting worse in each day.

#include <stdbool.h>
#include <stdio.h>

static bool is_valid_triangle(int i1, int i2, int i3)
{
    return (i1 + i2 > i3) && (i1 + i3 > i2) && (i2 + i3 > i1);
}

int main()
{
    int possible = 0;

    int i1, i2, i3;
    while (scanf("%d %d %d\n", &i1, &i2, &i3) == 3)
    {
        if (is_valid_triangle(i1, i2, i3))
            ++possible;
    }

    printf("%d possible triangles.\n", possible);

    return 0;
}

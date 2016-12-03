// Um... this is getting worse in each day.

#include <stdbool.h>
#include <stdio.h>

static bool is_valid_triangle(int i1, int i2, int i3)
{
    return (i1 + i2 > i3) && (i1 + i3 > i2) && (i2 + i3 > i1);
}

static char LINE_FORMAT[] = "%d %d %d\n";

int main()
{
    int possible = 0;

    int t1_i1, t1_i2, t1_i3;
    int t2_i1, t2_i2, t2_i3;
    int t3_i1, t3_i2, t3_i3;

    while (scanf(LINE_FORMAT, &t1_i1, &t2_i1, &t3_i1) == 3)
    {
        scanf(LINE_FORMAT, &t1_i2, &t2_i2, &t3_i2);
        scanf(LINE_FORMAT, &t1_i3, &t2_i3, &t3_i3);

        if (is_valid_triangle(t1_i1, t1_i2, t1_i3))
            ++possible;
        if (is_valid_triangle(t2_i1, t2_i2, t2_i3))
            ++possible;
        if (is_valid_triangle(t3_i1, t3_i2, t3_i3))
            ++possible;
    }

    printf("%d possible triangles.\n", possible);

    return 0;
}

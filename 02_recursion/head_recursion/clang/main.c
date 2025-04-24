#include <stdio.h>

void fn(int n)
{
    if (n > 0)
    {
        fn(n - 1);
        printf("%d ", n);
    }
}

int main()
{
    int n = 3;
    fn(n);
    return 0;
}
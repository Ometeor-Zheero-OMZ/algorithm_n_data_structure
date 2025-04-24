#include <stdio.h>

void fn(int n)
{
    if (n > 0)
    {
        printf("%d ", n);
        fn(n - 1);
    }
}

int main()
{
    int n = 3;
    fn(n);
    return 0;
}
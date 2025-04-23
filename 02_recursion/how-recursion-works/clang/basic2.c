#include <stdio.h>

void fn(int n)
{
    if (n > 0)
    {
        fn(n - 1);
        printf("%d ", n); // 1 2 3
    }
}

int main()
{
    int x = 3;

    fn(x);
    return 0;
}
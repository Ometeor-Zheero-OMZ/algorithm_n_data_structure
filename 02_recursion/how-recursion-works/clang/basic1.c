#include <stdio.h>

void fn(int n)
{
    if (n > 0)
    {
        printf("%d ", n); // 3 2 1
        fn(n - 1);
    }
}

int main()
{
    int x = 3;

    fn(x);
    return 0;
}
#include <stdio.h>
//#include "example_dll.h"

//g++ -shared -o example_dll.dll example_dll.o -Wl,--out-implib,libexample_dll.a

//__stdcall 
int hello(const char *s)
{
        printf("Hello2  %s\n", s);
        return 2;
}
#include <stdio.h>

int hello(const char *);

// The procedure entry point __divmoddi4 could not be located in the dynamic link library
// libgcc_s_dw2-1.dll was out of date

//hernad@DESKTOP-0HRJEDF mingw32 /c/Users/hernad/.rustup
//$ ls -l /c/Users/hernad/.rustup/toolchains/stable-i686-pc-windows-gnu/bin/libgcc_s_dw2-1.dll
// -rwxr-xr-x 1 hernad hernad 114190 Mar 30  2017 /c/Users/hernad/.rustup/toolchains/stable-i686-pc-windows-gnu/bin/libgcc_s_dw2-1.dll
//$ ls -l /c/msys64/mingw32/bin/libgcc_s_dw2-1.dll                                -rwxr-xr-x 1 hernad hernad 121460 Apr 25 12:36 /c/msys64/mingw32/bin/libgcc_s_dw2-1.dll
// -rwxr-xr-x 1 hernad hernad 121460 Apr 25 12:36 /c/msys64/mingw32/bin/libgcc_s_dw2-1.dll

//libgcc_s_dw2-1.dll
//libstdc++-6.dll
//libwinpthread-1.dll

char *hb_verPlatform( void );

/*
#include "hbapi.h"
#include "hbapiitm.h"

HB_FUNC( HBF1 )
{
    hb_retc("ok hbf1");
}

*/

int f1_c() {
    printf("hello=%d\n", hello("world"));
    return hello("w3");
}

int hb1_c() {
    printf("hb platform=%s", hb_verPlatform());
    return 55;
}
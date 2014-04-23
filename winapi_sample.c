#include <stdio.h>
#include "winapi/winapi.h"

int main() {
    HMODULE addr = GetModuleHandleW(NULL);
    printf("Module addres: 0x%x\n", addr);

    WindowContext context;
    context.width = 320;
    context.height = 240;
    context.handle = NULL;

    bool result = createWindow(&context, TEXT("Sample Window"));
    if (!result) {
        printf("Failed creating window!");
        return 1;
    }
    mainLoop(&context);
    return 0;
}
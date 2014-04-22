#ifndef WINAPI_H_
#define WINAPI_H_
#define UNICODE

#include <windows.h>
#include <stdbool.h>


#ifdef __cplusplus
extern "C" {
#endif

#define WINAPI_CALL_CONVENTION __stdcall

typedef struct {
	int width;
	int height;
	HWND handle;
} WindowContext;

bool WINAPI_CALL_CONVENTION createWindow(WindowContext* context, LPCTSTR title);
void WINAPI_CALL_CONVENTION mainLoop(WindowContext* context);

#ifdef __cplusplus
}
#endif

#endif // WINAPI_H_
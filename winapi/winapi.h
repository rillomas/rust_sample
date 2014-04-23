#ifndef WINAPI_H_
#define WINAPI_H_
#define UNICODE

#include <windows.h>
#include <stdbool.h>
#define API_CALL_CONVENTION __stdcall

#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
	int width;
	int height;
	HWND handle;
} WindowContext;

bool API_CALL_CONVENTION createWindow(WindowContext* context, LPCTSTR title);
void API_CALL_CONVENTION mainLoop(WindowContext* context);

#ifdef __cplusplus
}
#endif

#endif // WINAPI_H_
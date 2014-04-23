#ifndef WINAPI_H_
#define WINAPI_H_
#define UNICODE

#include <windows.h>
#include <stdbool.h>


#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
	int width;
	int height;
	HWND handle;
} WindowContext;

bool createWindow(WindowContext* context, LPCTSTR title);
void mainLoop(WindowContext* context);

#ifdef __cplusplus
}
#endif

#endif // WINAPI_H_
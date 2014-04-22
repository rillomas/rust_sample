#include "winapi.h"

static LRESULT WINAPI messageHandler(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
   LRESULT  lRet = 1; 
   switch (uMsg) 
   { 
      case WM_CREATE:
         break;

      case WM_SIZE:
         {
            // ESContext *esContext = (ESContext*)(LONG_PTR) GetWindowLongPtr ( hWnd, GWLP_USERDATA );
            // if ( esContext ) {
            //    esContext->width = LOWORD( lParam );
            //    esContext->height = HIWORD( lParam );
            //    InvalidateRect( esContext->hWnd, NULL, FALSE );
            // }
         }

      case WM_PAINT:
         {
            // ESContext *esContext = (ESContext*)(LONG_PTR) GetWindowLongPtr ( hWnd, GWLP_USERDATA );
            
            // if ( esContext && esContext->drawFunc )
            //    esContext->drawFunc ( esContext );
            
            // if ( esContext )
            //   ValidateRect( esContext->hWnd, NULL );
         }
         break;

      case WM_DESTROY:
         PostQuitMessage(0); 
         break; 
      
      case WM_CHAR:
         {
            // POINT      point;
            // ESContext *esContext = (ESContext*)(LONG_PTR) GetWindowLongPtr ( hWnd, GWLP_USERDATA );
            
            // GetCursorPos( &point );

            // if ( esContext && esContext->keyFunc )
               //  esContext->keyFunc ( esContext, (unsigned char) wParam, 
                  //                    (int) point.x, (int) point.y );
        }
         break;
         
      default: 
         lRet = DefWindowProc (hWnd, uMsg, wParam, lParam); 
         break; 
   } 

   return lRet; 
}


bool WINAPI_CALL_CONVENTION createWindow(WindowContext* context, LPCTSTR title) {
    WNDCLASS wnd;
    memset(&wnd, 0, sizeof(wnd));
    HINSTANCE hInstance = GetModuleHandle(NULL);
    wnd.lpfnWndProc   = (WNDPROC)messageHandler;
    wnd.style         = CS_OWNDC;
    wnd.hInstance     = hInstance; 
    wnd.hIcon = LoadIcon(NULL, IDI_APPLICATION);
    wnd.hCursor = LoadCursor(NULL, IDC_ARROW);
    wnd.hbrBackground = (HBRUSH)GetStockObject(BLACK_BRUSH); 
    wnd.lpszMenuName = NULL;
    wnd.lpszClassName = TEXT("opengles2.0");

    if (!RegisterClass (&wnd) ) {
        return false; 
    }

    DWORD wStyle = WS_VISIBLE | WS_POPUP | WS_BORDER | WS_SYSMENU | WS_CAPTION | WS_SIZEBOX;

    // Adjust the window rectangle so that the client area has
    // the correct number of pixels
    RECT windowRect;
    windowRect.left = 0;
    windowRect.top = 0;
    windowRect.right = context->width;
    windowRect.bottom = context->height;

    // BOOL result = AdjustWindowRect(&windowRect, wStyle, FALSE);
    // if (!result) {
    //     return false;
    // }

    HWND handle = CreateWindow(
                        TEXT("opengles2.0"),
                        title,
                        wStyle,
                        0,
                        0,
                        windowRect.right - windowRect.left,
                        windowRect.bottom - windowRect.top,
                        NULL,
                        NULL,
                        hInstance,
                        NULL);

    // Set the ESContext* to the GWL_USERDATA so that it is available to the 
    // ESWindowProc
    SetWindowLongPtr(handle, GWLP_USERDATA, (LONG) (LONG_PTR) NULL);

    if(handle == NULL) {
      return false;
    }

    ShowWindow(handle, TRUE);
    context->handle = handle;
    return true;
}

void WINAPI_CALL_CONVENTION mainLoop(WindowContext* context) {
    MSG msg = { 0 };
    int done = 0;
    // DWORD lastTime = GetTickCount();

    while (!done) {
        int gotMsg = (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE) != 0);
        // DWORD curTime = GetTickCount();
        // float deltaTime = (float)( curTime - lastTime ) / 1000.0f;
        // lastTime = curTime;

        if ( gotMsg ) {
            if (msg.message==WM_QUIT) {
                done=1; 
            } else {
                TranslateMessage(&msg); 
                DispatchMessage(&msg); 
            }
        } else {
            SendMessage( context->handle, WM_PAINT, 0, 0 );
        }

        // Call update function if registered
        // if (esContext->updateFunc != NULL ) {
        //     esContext->updateFunc ( esContext, deltaTime );
        // }
    }
}


#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

typedef void(CALLBACK *PHANDLED_PROC)(PVOID Closure);

typedef struct _EXCEPTION
{
    DWORD Code;
    PVOID Address;
} EXCEPTION, *PEXCEPTION;

BOOL HandlerStub(_In_ PHANDLED_PROC HandledProc, _In_ PVOID Closure, _Inout_ PEXCEPTION Exception)
{
    BOOL Success = TRUE;
    LPEXCEPTION_POINTERS Pointers = NULL;
    DWORD Code = 0;

    __try
    {
        HandledProc(Closure);
    }
    __except (Code = GetExceptionCode(), Pointers = GetExceptionInformation(), EXCEPTION_EXECUTE_HANDLER)
    {
        Success = FALSE;
        if (Exception != NULL)
        {
            // Use GetExceptionCode() instead of Record->ExceptionCode as it is more reliable.
            Exception->Code = Code;

            PEXCEPTION_RECORD Record = Pointers->ExceptionRecord;
            Exception->Address = Record->ExceptionAddress;
        }
    }

    return Success;
}

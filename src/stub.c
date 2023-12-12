#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

typedef void(CALLBACK *PHANDLED_PROC)(PVOID Closure);

typedef struct _EXCEPTION
{
    DWORD Code;
    PVOID Address;
#ifdef _WIN64
    DWORD64 regs[17];
#else
    DWORD32 regs[9];
#endif
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

#ifdef _WIN64
            Exception->regs[0] = Pointers->ContextRecord->Rax;
            Exception->regs[1] = Pointers->ContextRecord->Rcx;
            Exception->regs[2] = Pointers->ContextRecord->Rdx;
            Exception->regs[3] = Pointers->ContextRecord->Rbx;
            Exception->regs[4] = Pointers->ContextRecord->Rsp;
            Exception->regs[5] = Pointers->ContextRecord->Rbp;
            Exception->regs[6] = Pointers->ContextRecord->Rsi;
            Exception->regs[7] = Pointers->ContextRecord->Rdi;
            Exception->regs[8] = Pointers->ContextRecord->R8;
            Exception->regs[9] = Pointers->ContextRecord->R9;
            Exception->regs[10] = Pointers->ContextRecord->R10;
            Exception->regs[11] = Pointers->ContextRecord->R11;
            Exception->regs[12] = Pointers->ContextRecord->R12;
            Exception->regs[13] = Pointers->ContextRecord->R13;
            Exception->regs[14] = Pointers->ContextRecord->R14;
            Exception->regs[15] = Pointers->ContextRecord->R15;
            Exception->regs[15] = Pointers->ContextRecord->Rip;
#else
            Exception->regs[0] = Pointers->ContextRecord->Eax;
            Exception->regs[1] = Pointers->ContextRecord->Ecx;
            Exception->regs[2] = Pointers->ContextRecord->Edx;
            Exception->regs[3] = Pointers->ContextRecord->Ebx;
            Exception->regs[4] = Pointers->ContextRecord->Esp;
            Exception->regs[5] = Pointers->ContextRecord->Ebp;
            Exception->regs[6] = Pointers->ContextRecord->Esi;
            Exception->regs[7] = Pointers->ContextRecord->Edi;
            Exception->regs[8] = Pointers->ContextRecord->Eip;
#endif
        }
    }

    return Success;
}

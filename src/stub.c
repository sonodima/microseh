#define WIN32_LEAN_AND_MEAN
#include <windows.h>

#define TG_ARCH_X86 1
#define TG_ARCH_X64 2
#define TG_ARCH_ARM64 3
#define TG_ARCH_UNKNOWN 4

#if defined(_M_IX86)
#define TG_ARCH TG_ARCH_X86
#elif defined(_M_X64) || defined(_M_AMD64)
#define TG_ARCH TG_ARCH_X64
#elif defined(_M_ARM64)
#define TG_ARCH TG_ARCH_ARM64
#else
#define TG_ARCH TG_ARCH_UNKNOWN
#endif


#define HAS_REGISTERS TG_ARCH != TG_ARCH_UNKNOWN

#if HAS_REGISTERS

#if TG_ARCH == TG_ARCH_X86
#define NUM_REGISTERS 9
#elif TG_ARCH == TG_ARCH_X64
#define NUM_REGISTERS 17
#elif TG_ARCH == TG_ARCH_ARM64
#define NUM_REGISTERS 33
#endif

typedef struct _REGISTERS
{
    DWORD_PTR List[NUM_REGISTERS];
} REGISTERS, *PREGISTERS;

REGISTERS MakeRegisters(_In_ PCONTEXT Context)
{
    REGISTERS Registers;

#if TG_ARCH == TG_ARCH_X86
    Registers.List[0] = Context->Eax;
    Registers.List[1] = Context->Ecx;
    Registers.List[2] = Context->Edx;
    Registers.List[3] = Context->Ebx;
    Registers.List[4] = Context->Esp;
    Registers.List[5] = Context->Ebp;
    Registers.List[6] = Context->Esi;
    Registers.List[7] = Context->Edi;
    Registers.List[8] = Context->Eip;
#elif TG_ARCH == TG_ARCH_X64
    Registers.List[0] = Context->Rax;
    Registers.List[1] = Context->Rcx;
    Registers.List[2] = Context->Rdx;
    Registers.List[3] = Context->Rbx;
    Registers.List[4] = Context->Rsp;
    Registers.List[5] = Context->Rbp;
    Registers.List[6] = Context->Rsi;
    Registers.List[7] = Context->Rdi;
    Registers.List[8] = Context->R8;
    Registers.List[9] = Context->R9;
    Registers.List[10] = Context->R10;
    Registers.List[11] = Context->R11;
    Registers.List[12] = Context->R12;
    Registers.List[13] = Context->R13;
    Registers.List[14] = Context->R14;
    Registers.List[15] = Context->R15;
    Registers.List[16] = Context->Rip;
#elif TG_ARCH == TG_ARCH_ARM64
    Registers.List[0] = Context->X0;
    Registers.List[1] = Context->X1;
    Registers.List[2] = Context->X2;
    Registers.List[3] = Context->X3;
    Registers.List[4] = Context->X4;
    Registers.List[5] = Context->X5;
    Registers.List[6] = Context->X6;
    Registers.List[7] = Context->X7;
    Registers.List[8] = Context->X8;
    Registers.List[9] = Context->X9;
    Registers.List[10] = Context->X10;
    Registers.List[11] = Context->X11;
    Registers.List[12] = Context->X12;
    Registers.List[13] = Context->X13;
    Registers.List[14] = Context->X14;
    Registers.List[15] = Context->X15;
    Registers.List[16] = Context->X16;
    Registers.List[17] = Context->X17;
    Registers.List[18] = Context->X18;
    Registers.List[19] = Context->X19;
    Registers.List[20] = Context->X20;
    Registers.List[21] = Context->X21;
    Registers.List[22] = Context->X22;
    Registers.List[23] = Context->X23;
    Registers.List[24] = Context->X24;
    Registers.List[25] = Context->X25;
    Registers.List[26] = Context->X26;
    Registers.List[27] = Context->X27;
    Registers.List[28] = Context->X28;
    Registers.List[29] = Context->Fp;
    Registers.List[30] = Context->Lr;
    Registers.List[31] = Context->Sp;
    Registers.List[32] = Context->Pc;
#endif

    return Registers;
}

#endif


typedef void(CALLBACK *PHANDLED_PROC)(PVOID Closure);

typedef struct _EXCEPTION
{
    DWORD Code;
    PVOID Address;
#if HAS_REGISTERS
    REGISTERS Registers;
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
            Exception->Address = Pointers->ExceptionRecord->ExceptionAddress;
#if HAS_REGISTERS
            Exception->Registers = MakeRegisters(Pointers->ContextRecord);
#endif
        }
    }

    return Success;
}

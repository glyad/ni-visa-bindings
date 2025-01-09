#ifdef _WIN32
    #include "C:\\Program Files (x86)\\IVI Foundation\\VISA\\WinNT\\Include\\visa.h" // Windows path
#elif __APPLE__
    #include "/Library/Frameworks/VISA.framework/Headers/visa.h" // macOS path
#elif __linux__
    #include "/usr/include/visa.h" // Linux path
#else
    #error "Unsupported platform"
#endif

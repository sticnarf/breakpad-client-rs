#include <string>
#include <cstdio>
#include "breakpad_c.h"

void register_handler_from_path(const char *c_path,
                                FilterCallback filter,
                                MinidumpCallback callback,
                                void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(c_path);
    // Prevent calling its destructure
    new google_breakpad::ExceptionHandler(descriptor, filter, callback, callback_context, true, -1);
}

void register_handler_from_fd(int fd,
                              FilterCallback filter,
                              MinidumpCallback callback,
                              void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(fd);
    // Prevent calling its destructure
    new google_breakpad::ExceptionHandler(descriptor, filter, callback, callback_context, true, -1);
}
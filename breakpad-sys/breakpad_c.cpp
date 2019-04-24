#include <string>
#include "breakpad_c.h"

void register_handler_from_path(const char *c_path,
                                FilterCallback filter,
                                MinidumpCallback callback,
                                void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(c_path);
    google_breakpad::ExceptionHandler eh(descriptor, filter, callback, callback_context, true, -1);
}

void register_handler_from_fd(int fd,
                              FilterCallback filter,
                              MinidumpCallback callback,
                              void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(fd);
    google_breakpad::ExceptionHandler eh(descriptor, filter, callback, callback_context, true, -1);
}
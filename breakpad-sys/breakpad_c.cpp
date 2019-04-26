#include <string>
#include <cstdio>
#include "breakpad_c.h"

struct CallbackContext {
    FilterCallback filter;
    WrappedMinidumpCallback callback;
    void *context;
};

bool __minidump_callback(const google_breakpad::MinidumpDescriptor &descriptor,
                         void *context,
                         bool succeeded) {
    CallbackContext *ctx = (CallbackContext *) context;
    DescriptorInfo info;
    info.c_path = descriptor.path();
    return ctx->callback(info, ctx->context, succeeded);
}

void register_handler_from_path(const char *c_path,
                                FilterCallback filter,
                                WrappedMinidumpCallback callback,
                                void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(c_path);
    CallbackContext *ctx = new CallbackContext;
    ctx->filter = filter;
    ctx->callback = callback;
    ctx->context = callback_context;
    // Prevent calling its destructure
    new google_breakpad::ExceptionHandler(descriptor, NULL, __minidump_callback, (void *)ctx, true, -1);
}

//void register_handler_from_fd(int fd,
//                              FilterCallback filter,
//                              MinidumpCallback callback,
//                              void *callback_context) {
//    google_breakpad::MinidumpDescriptor descriptor(fd);
//    // Prevent calling its destructure
//    new google_breakpad::ExceptionHandler(descriptor, filter, callback, callback_context, true, -1);
//}
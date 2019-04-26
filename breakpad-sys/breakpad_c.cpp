#include <string>
#include <cstdio>
#include "breakpad_c.h"

struct CallbackContext {
    FilterCallback filter;
    WrappedMinidumpCallback callback;
    void *context;
};

static bool __filter_callback(void *context) {
    CallbackContext *ctx = (CallbackContext *) context;
    return ctx->filter(ctx->context);
}

static bool __minidump_callback(const google_breakpad::MinidumpDescriptor &descriptor,
                         void *context,
                         bool succeeded) {
    CallbackContext *ctx = (CallbackContext *) context;
    DescriptorInfo info;
    info.c_path = descriptor.path();
    return ctx->callback(info, ctx->context, succeeded);
}

static void register_handler(const google_breakpad::MinidumpDescriptor &descriptor,
                        FilterCallback filter,
                        WrappedMinidumpCallback callback,
                        void *callback_context) {
    CallbackContext *ctx = new CallbackContext;
    ctx->filter = filter;
    ctx->callback = callback;
    ctx->context = callback_context;

    // Prevent calling its destructure
    new google_breakpad::ExceptionHandler(descriptor, __filter_callback, __minidump_callback, (void *)ctx, true, -1);
}

extern "C" void register_handler_from_path(const char *c_path,
                                FilterCallback filter,
                                WrappedMinidumpCallback callback,
                                void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(c_path);
    register_handler(descriptor, filter, callback, callback_context);
}

extern "C" void register_handler_from_fd(int fd,
                                FilterCallback filter,
                                WrappedMinidumpCallback callback,
                                void *callback_context) {
    google_breakpad::MinidumpDescriptor descriptor(fd);
    register_handler(descriptor, filter, callback, callback_context);
}
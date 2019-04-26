/// C Wrapper of breakpad client

#include "client/linux/handler/exception_handler.h"

extern "C" {

/// This struct stores information in MinidumpDescriptor.
/// Thus, we can pass MinidumpDescriptor to an entirely C environment
struct DescriptorInfo {
    const char *c_path;
    // More fields may be added in the future
};

// The following two types are extracted from breakpad client headers.
// They are the callback types when registering an exception handler.

// FilterCallback can be used in the C environment, while MinidumpCallback cannot.
// So WrappedMinidumpCallback is a wrapper callback function type that converts
// C++ MinidumpDescriptor to DescriptorInfo which can be used in C, then we use
// the wrapped callback function in the C environment.

typedef bool (*FilterCallback)(void *context);

typedef bool (*MinidumpCallback)(const google_breakpad::MinidumpDescriptor &descriptor,
                                 void *context,
                                 bool succeeded);

typedef bool (*WrappedMinidumpCallback)(const DescriptorInfo descriptor,
                                        void *context,
                                        bool succeeded);

void register_handler_from_path(const char *c_path,
                                FilterCallback filter,
                                WrappedMinidumpCallback callback,
                                void *callback_context);

void register_handler_from_fd(int fd,
                              FilterCallback filter,
                              WrappedMinidumpCallback callback,
                              void *callback_context);
}


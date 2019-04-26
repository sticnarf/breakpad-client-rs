/// C Wrapper of breakpad client

#include "client/linux/handler/exception_handler.h"

extern "C" {
struct DescriptorInfo {
    const char *c_path;
};

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

//void register_handler_from_fd(int fd,
//                              FilterCallback filter,
//                              WrappedMinidumpCallback callback,
//                              void *callback_context);
}


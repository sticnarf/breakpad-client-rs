/// C Wrapper of breakpad client

#include "client/linux/handler/exception_handler.h"

typedef bool (*FilterCallback)(void *context);
typedef bool (*MinidumpCallback)(const google_breakpad::MinidumpDescriptor &descriptor,
                                 void *context,
                                 bool succeeded);

void register_handler_from_path(const char *c_path,
                      FilterCallback filter,
                      MinidumpCallback callback,
                      void *callback_context);

void register_handler_from_fd(int fd,
                                FilterCallback filter,
                                MinidumpCallback callback,
                                void *callback_context);
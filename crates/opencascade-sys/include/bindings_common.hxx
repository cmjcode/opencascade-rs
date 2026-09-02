#pragma once
#include "rust/cxx.h"
#include <NCollection_List.hxx>
#include <Standard_Failure.hxx>
#include <memory>
#include <cstdio>
#ifdef _WIN32
#include <io.h>
#include <fcntl.h>
#else
#include <unistd.h>
#include <fcntl.h>
#endif

struct CoutSilencer {
  int old_stdout;
  int old_stderr;
  int null_fd;
  bool active;

  CoutSilencer() : old_stdout(-1), old_stderr(-1), null_fd(-1), active(false) {
    std::fflush(stdout);
    std::fflush(stderr);
#ifdef _WIN32
    null_fd = _open("NUL", _O_WRONLY);
    if (null_fd >= 0) {
      old_stdout = _dup(1);
      old_stderr = _dup(2);
      if (old_stdout >= 0 && old_stderr >= 0) {
        _dup2(null_fd, 1);
        _dup2(null_fd, 2);
        active = true;
      }
    }
#else
    null_fd = open("/dev/null", O_WRONLY);
    if (null_fd >= 0) {
      old_stdout = dup(STDOUT_FILENO);
      old_stderr = dup(STDERR_FILENO);
      if (old_stdout >= 0 && old_stderr >= 0) {
        dup2(null_fd, STDOUT_FILENO);
        dup2(null_fd, STDERR_FILENO);
        active = true;
      }
    }
#endif
  }

  ~CoutSilencer() {
    if (active) {
      std::fflush(stdout);
      std::fflush(stderr);
#ifdef _WIN32
      _dup2(old_stdout, 1);
      _dup2(old_stderr, 2);
      _close(old_stdout);
      _close(old_stderr);
      _close(null_fd);
#else
      dup2(old_stdout, STDOUT_FILENO);
      dup2(old_stderr, STDERR_FILENO);
      close(old_stdout);
      close(old_stderr);
      close(null_fd);
#endif
    }
  }
};

[[noreturn]] inline void rethrow_standard_failure_as_runtime_error(const Standard_Failure &failure,
                                                                    const char *fallback_message) {
  const char *message = failure.GetMessageString();
  throw std::runtime_error((message != nullptr && message[0] != '\0') ? message : fallback_message);
}

// Generic template constructor
template <typename T, typename... Args> std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}

// Type casting
template <typename T, typename U> inline U upcast(T src) { return src; }
template <typename T, typename U> inline const U &upcast_ref(const T &src) { return src; }

// Generic List
template <typename T> std::unique_ptr<std::vector<T>> list_to_vector(const NCollection_List<T> &list) {
  return std::unique_ptr<std::vector<T>>(new std::vector<T>(list.begin(), list.end()));
}

#include <type_traits>
#include <utility>

template <typename HandleT>
inline const typename std::remove_reference<decltype(*std::declval<HandleT>())>::type &
handle_try_deref(const HandleT &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

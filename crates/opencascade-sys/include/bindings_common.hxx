#pragma once
#include "rust/cxx.h"
#include <NCollection_List.hxx>
#include <Standard_Failure.hxx>
#include <memory>

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

template <typename T> const T &handle_try_deref(const opencascade::handle<T> &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

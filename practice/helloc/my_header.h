#pragma once

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace top {
namespace data {

using namespace std;

extern "C" {

void free_str(char *s);

const char *hello(const char *name);

const char *hello_bad(const char *name);

const char *hello_world();

} // extern "C"

} // namespace data
} // namespace top

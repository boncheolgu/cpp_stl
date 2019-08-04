#include <memory>
#include <string>
#include <vector>

#define VECTOR_OF(T)                                                           \
  struct alignas(alignof(std::vector<T>)) vector_of_##T {                      \
    uint8_t payload[sizeof(std::vector<T>)];                                   \
  }

namespace rust {
VECTOR_OF(bool);
VECTOR_OF(uint8_t);
VECTOR_OF(uint16_t);
VECTOR_OF(uint32_t);
VECTOR_OF(uint64_t);
VECTOR_OF(int8_t);
VECTOR_OF(int16_t);
VECTOR_OF(int32_t);
VECTOR_OF(int64_t);

struct alignas(alignof(std::vector<int32_t>)) vector_of_i32 {
  uint8_t payload[sizeof(std::vector<int32_t>)];
};

struct alignas(alignof(std::vector<std::unique_ptr<void>>))
    vector_of_unique_ptr {
  uint8_t payload[sizeof(std::vector<std::unique_ptr<void>>)];
};
} // namespace rust
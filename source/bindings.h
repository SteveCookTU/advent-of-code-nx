#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>



extern "C" {

void free_result(char *s);

void init_heap();

char *run_day(int32_t year, int32_t day, char* input);

} // extern "C"

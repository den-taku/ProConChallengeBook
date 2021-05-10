// write signiture as prototype declaration.
#include <tuple>
#include <vector>

bool condition(std::vector<std::tuple<int, int>> v, int k, int est);

int binary_search(std::vector<std::tuple<int, int>> v, int k, int lower_bound,
                  int upper_bound);

double calculate(std::vector<std::tuple<int, int>> v, int k, int est);
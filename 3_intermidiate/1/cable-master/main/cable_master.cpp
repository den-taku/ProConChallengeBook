// Let's start coding!
// Coder: DenTaku
#include <vector>

void somefunc() {
  //
}

int cut(std::vector<int> v, int est) {
  int num = 0;
  for (const auto &e : v) {
    num += e / est;
  }
  return num;
}

int b_search(std::vector<int> v, int lower_bound, int upper_bound, int k) {
  if (upper_bound - lower_bound == 1) {
    return lower_bound;
  }
  int est = lower_bound + (upper_bound - lower_bound) / 2;
  int count = cut(v, est);
  if (count < k) {
    return b_search(v, lower_bound, est, k);
  } else if (count >= k) {
    return b_search(v, est, upper_bound, k);
  }
}
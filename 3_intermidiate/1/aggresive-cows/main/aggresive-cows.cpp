// Let's start coding!
// Coder: DenTaku
#include <vector>

// est の時，そのような入れ方は存在するか
bool condition(int est, std::vector<int> v, int m) {
  for(int prior = 0, i = 1; i < m; ++i) {
    int next = prior + 1;
    while(v[next] - v[prior] < est && next < v.size()) {
      ++next;
    }
    if (next == v.size()) {
      return false;
    } else {
      prior = next;
    }
  }
  return true;
}

int binary_search(int lower_bound, int upper_bound, std::vector<int> v, int n, int m) {
  if (upper_bound - lower_bound == 1) {
    if (condition(upper_bound, v, m)) {
      return upper_bound;
    } else {
      return lower_bound;
    }
  }
  int est = (upper_bound + lower_bound) / 2;
  if (condition(est, v, m)) {
    return binary_search(est, upper_bound, v, n, m);
  } else {
    return binary_search(lower_bound, est, v, n, m);
  }
}

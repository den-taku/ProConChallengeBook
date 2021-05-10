// Let's start coding!
// Coder: DenTaku
#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

double calculate(std::vector<std::tuple<int, int>> v, int k, int x) {
  std::cout << "est: " << x << std::endl;
  std::vector<std::tuple<int, size_t>> u = {};
  for (size_t i = 0; i < v.size(); ++i) {
    u.push_back(std::make_tuple(std::get<0>(v[i]) - x * std::get<1>(v[i]), i));
  }
  std::sort(u.begin(), u.end());
  std::for_each(u.begin(), u.end(), [](std::tuple<int, size_t> e) {
    std::cout << std::get<0>(e) << " " << std::get<1>(e) << std::endl;
  });
  std::cout << std::endl;
  double v_sum = 0;
  double w_sum = 0;
  for (int i = 0; i < k; ++i) {
    size_t index = std::get<1>(u[u.size() - i]);
    v_sum += std::get<0>(v[index]);
    w_sum += std::get<1>(v[index]);
  }
  return v_sum / w_sum;
}

bool condition(std::vector<std::tuple<int, int>> v, int k, int est) {
  std::vector<std::tuple<int, size_t>> u = {};
  for (size_t i = 0; i < v.size(); ++i) {
    u.push_back(
        std::make_tuple(std::get<0>(v[i]) - est * std::get<1>(v[i]), i));
  }
  std::sort(u.begin(), u.end());
  double sum_cond = 0;
  for (int i = 0; i < k; ++i) {
    sum_cond += std::get<0>(u[u.size() - i]);
  }
  return sum_cond >= 0;
}

int binary_search(std::vector<std::tuple<int, int>> v, int k, int lower_bound,
                  int upper_bound) {
  if (upper_bound - lower_bound == 1) {
    if (condition(v, k, upper_bound)) {
      return upper_bound;
    } else {
      return lower_bound;
    }
  } else {
    int est = (upper_bound + lower_bound) / 2;
    if (condition(v, k, est)) {
      return binary_search(v, k, est, upper_bound);
    } else {
      return binary_search(v, k, lower_bound, est);
    }
  }
}

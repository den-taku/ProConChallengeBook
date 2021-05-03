#include "lib/proconlib.h"
#include "lower_bound.h"
#include <algorithm>
#include <iostream>
#include <vector>
// using namespace std;

int main() {
  int n = 0;
  std::cin >> n;
  std::vector<int> v = {};

  int tmp = 0;
  for (int i = 0; i < n; ++i) {
    std::cin >> tmp;
    v.push_back(tmp);
  }
  std::sort(v.begin(), v.end(), std::greater<int>{});

  int k = 0;
  std::cin >> k;

  std::cout << 1 + b_search(v, k, 0, v.size() - 1) << std::endl;
  return 0;
}

#include "cable_master.h"
#include "lib/proconlib.h"
#include <algorithm>
#include <climits>
#include <iostream>
#include <vector>
// using namespace std;

int main() {
  int n = 0;
  int k = 0;
  std::vector<int> v = {};
  int min = INT_MAX;
  double tmp = 0;
  int casted = 0;

  std::cin >> n;
  std::cin >> k;
  for (int i = 0; i < n; ++i) {
    std::cin >> tmp;
    casted = static_cast<int>(tmp * 100.0);
    if (casted < min) {
      min = casted;
    }
    v.push_back(casted);
  }

  std::cout << static_cast<double>(b_search(v, 0, min * 2, k)) / 100.0
            << std::endl;
  return 0;
}

#include "aggresive-cows.h"
#include "lib/proconlib.h"
#include <algorithm>
#include <iostream>
#include <vector>
#include <climits>
// using namespace std;

int main() {
  using namespace std;
  int n = 0;
  int m = 0;
  cin >> n;
  cin >> m;
  vector<int> v = {};
  int tmp = 0;
  for (int i = 0; i < n; ++i) {
    cin >> tmp;
    v.push_back(tmp);
  }
  sort(v.begin(), v.end());
  cout << binary_search(1, v[n-1] / m, v, n, m) << endl;
  return 0;
}

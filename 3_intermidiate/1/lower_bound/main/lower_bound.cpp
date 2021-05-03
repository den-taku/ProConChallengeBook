// Coder: DenTaku
// #include <iostream>
#include <vector>

void somefunc() {
  //
}

int b_search(std::vector<int> v, int k, int begin, int end) {
  // std::cout << end - begin << std::endl;
  if (end - begin == 0) {
    return begin;
  } else if (end - begin == 1) {
    if (v[begin] < k) {
      return end;
    } else {
      return begin;
    }
  } else {
    int pivot = v[(end - begin) / 2 + begin];
    if (pivot < k) {
      return b_search(v, k, (end - begin) / 2 + begin, end);
    } else {
      return b_search(v, k, begin, (end - begin) / 2 + begin);
    }
  }
}
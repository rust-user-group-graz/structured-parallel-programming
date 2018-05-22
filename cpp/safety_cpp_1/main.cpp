#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<string> vector;
    vector.push_back("one");
    auto& elem = vector[0];
    vector.push_back("two");
    cout << elem;
    return 0;
}

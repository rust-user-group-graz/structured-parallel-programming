#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<string> vector;
    auto& elem = vector[0];
    vector.push_back("test");
    cout << elem;
    return 0;
}

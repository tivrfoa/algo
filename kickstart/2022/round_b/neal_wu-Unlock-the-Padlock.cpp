#include <algorithm>
#include <array>
#include <bitset>
#include <cassert>
#include <chrono>
#include <cmath>
#include <cstring>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <numeric>
#include <queue>
#include <random>
#include <set>
#include <vector>
using namespace std;

// http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2016/p0200r0.html
template<class Fun> class y_combinator_result {
    Fun fun_;
public:
    template<class T> explicit y_combinator_result(T &&fun): fun_(std::forward<T>(fun)) {}
    template<class ...Args> decltype(auto) operator()(Args &&...args) { return fun_(std::ref(*this), std::forward<Args>(args)...); }
};
template<class Fun> decltype(auto) y_combinator(Fun &&fun) { return y_combinator_result<std::decay_t<Fun>>(std::forward<Fun>(fun)); }


template<typename A, typename B> ostream& operator<<(ostream &os, const pair<A, B> &p) { return os << '(' << p.first << ", " << p.second << ')'; }
template<typename T_container, typename T = typename enable_if<!is_same<T_container, string>::value, typename T_container::value_type>::type> ostream& operator<<(ostream &os, const T_container &v) { os << '{'; string sep; for (const T &x : v) os << sep << x, sep = ", "; return os << '}'; }

void dbg_out() { cerr << endl; }
template<typename Head, typename... Tail> void dbg_out(Head H, Tail... T) { cerr << ' ' << H; dbg_out(T...); }
#ifdef NEAL_DEBUG
#define dbg(...) cerr << "(" << #__VA_ARGS__ << "):", dbg_out(__VA_ARGS__)
#else
#define dbg(...)
#endif


const int64_t INF64 = int64_t(2e18) + 5;

void run_case(int test_case) {
    int N, D;
    cin >> N >> D;
    vector<int> A(N);

    for (auto &a : A)
        cin >> a;

    vector<vector<array<int64_t, 2>>> dp(N + 1, vector<array<int64_t, 2>>(N + 1, {-1, -1}));

    auto distance = [&](int x, int y) -> int {
        int diff = abs(x - y);
        return min(diff, D - diff);
    };

    auto solve = y_combinator([&](auto self, int start, int end, int side) -> int64_t {
        int value = side == 0 ? A[start] : A[end - 1];

        if (start == 0 && end == N)
            return distance(value, 0);

        int64_t &ans = dp[start][end][side];

        if (ans >= 0)
            return ans;

        ans = INF64;

        if (start > 0)
            ans = min(ans, distance(value, A[start - 1]) + self(start - 1, end, 0));

        if (end < N)
            ans = min(ans, distance(value, A[end]) + self(start, end + 1, 1));

        return ans;
    });

    int64_t best = INF64;

    for (int i = 0; i < N; i++)
        best = min(best, solve(i, i + 1, 0));

    cout << "Case #" << test_case << ": " << best << '\n';
}

int main() {
    int tests;
    cin >> tests;

    for (int tc = 1; tc <= tests; tc++) {
        run_case(tc);
        cout << flush;
    }
}

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

struct union_find {
    vector<int> parent;
    vector<int> size;
    int components = 0;

    union_find(int n = -1) {
        if (n >= 0)
            init(n);
    }

    void init(int n) {
        parent.resize(n + 1);
        size.assign(n + 1, 1);
        components = n;

        for (int i = 0; i <= n; i++)
            parent[i] = i;
    }

    int find(int x) {
        return x == parent[x] ? x : find(parent[x]);
    }

    bool unite(int x, int y) {
        x = find(x);
        y = find(y);

        if (x == y)
            return false;

        if (size[x] < size[y])
            swap(x, y);

        parent[y] = x;
        size[x] += size[y];
        components--;
        return true;
    }
};


void run_case(int test_case) {
    int R, C;
    cin >> R >> C;
    vector<string> grid(R);

    for (auto &row : grid)
        cin >> row;

    auto encode = [&](int r, int c) -> int {
        assert(0 <= r && r < R && 0 <= c && c < C);
        return r * C + c;
    };

    union_find UF(R * C);
    int empty = 0;

    for (int r = 0; r < R; r++)
        for (int c = 0; c < C; c++) {
            if (grid[r][c] != '*') {
                empty++;
                continue;
            }

            if (r + 1 < R && grid[r + 1][c] == '*')
                UF.unite(encode(r, c), encode(r + 1, c));

            if (c + 1 < C && grid[r][c + 1] == '*')
                UF.unite(encode(r, c), encode(r, c + 1));
        }

    if (UF.components != empty + 1) {
        cout << "Case #" << test_case << ": ";
        cout << "IMPOSSIBLE" << '\n';
        return;
    }

    auto double_encode = [&](int r, int c) -> int {
        assert(0 <= r && r < 2 * R && 0 <= c && c < 2 * C);
        return r * (2 * C) + c;
    };

    vector<set<int>> double_adj(4 * R * C);

    auto add_edge = [&](int x, int y) -> void {
        double_adj[x].insert(y);
        double_adj[y].insert(x);
    };

    auto erase_edge = [&](int x, int y) -> void {
        double_adj[x].erase(y);
        double_adj[y].erase(x);
    };

    for (int r = 0; r < R; r++)
        for (int c = 0; c < C; c++)
            if (grid[r][c] == '*') {
                add_edge(double_encode(2 * r, 2 * c), double_encode(2 * r, 2 * c + 1));
                add_edge(double_encode(2 * r, 2 * c), double_encode(2 * r + 1, 2 * c));
                add_edge(double_encode(2 * r, 2 * c + 1), double_encode(2 * r + 1, 2 * c + 1));
                add_edge(double_encode(2 * r + 1, 2 * c), double_encode(2 * r + 1, 2 * c + 1));
            }

    UF.init(R * C);

    for (int r = 0; r < R; r++)
        for (int c = 0; c < C; c++) {
            if (grid[r][c] != '*')
                continue;

            if (r + 1 < R && grid[r + 1][c] == '*' && UF.unite(encode(r, c), encode(r + 1, c))) {
                int br = r, bc = c;
                r++;
                assert(c == bc);
                erase_edge(double_encode(2 * br + 1, 2 * bc), double_encode(2 * br + 1, 2 * bc + 1));
                erase_edge(double_encode(2 * r, 2 * c), double_encode(2 * r, 2 * c + 1));
                add_edge(double_encode(2 * br + 1, 2 * bc), double_encode(2 * r, 2 * c));
                add_edge(double_encode(2 * br + 1, 2 * bc + 1), double_encode(2 * r, 2 * c + 1));
                r--;
            }

            if (c + 1 < C && grid[r][c + 1] == '*' && UF.unite(encode(r, c), encode(r, c + 1))) {
                int br = r, bc = c;
                c++;
                assert(r == br);
                erase_edge(double_encode(2 * br, 2 * bc + 1), double_encode(2 * br + 1, 2 * bc + 1));
                erase_edge(double_encode(2 * r, 2 * c), double_encode(2 * r + 1, 2 * c));
                add_edge(double_encode(2 * br, 2 * bc + 1), double_encode(2 * r, 2 * c));
                add_edge(double_encode(2 * br + 1, 2 * bc + 1), double_encode(2 * r + 1, 2 * c));
                c--;
            }
        }

    string solution;

    auto dfs = y_combinator([&](auto self, int node) -> void {
        if (double_adj[node].empty())
            return;

        int neigh = *double_adj[node].begin();
        double_adj[node].erase(neigh);
        double_adj[neigh].erase(node);

        if (abs(neigh - node) == 1)
            solution += neigh > node ? 'E' : 'W';
        else
            solution += neigh > node ? 'S' : 'N';

        self(neigh);
    });

    dfs(0);

    cout << "Case #" << test_case << ": ";
    cout << solution << '\n';
}

int main() {
    int tests;
    cin >> tests;

    for (int tc = 1; tc <= tests; tc++) {
        run_case(tc);
        cout << flush;
    }
}

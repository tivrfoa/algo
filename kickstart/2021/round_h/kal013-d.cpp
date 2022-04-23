/*/ Author: kal013 /*/
// #pragma GCC optimize ("O3")
#include "bits/stdc++.h"
#include "ext/pb_ds/assoc_container.hpp"
#include "ext/pb_ds/tree_policy.hpp"
using namespace std;
using namespace __gnu_pbds;

template<class T> 
using ordered_set = tree<T, null_type,less<T>, rb_tree_tag, tree_order_statistics_node_update> ;

template<class key, class value, class cmp = std::less<key>>
using ordered_map = tree<key, value, cmp, rb_tree_tag, tree_order_statistics_node_update>;
// find_by_order(k)  returns iterator to kth element starting from 0;
// order_of_key(k) returns count of elements strictly smaller than k;

template<class T>
using min_heap = priority_queue<T,vector<T>,greater<T> >; 

/*/---------------------------IO(Debugging)----------------------/*/
template<class T> istream& operator >> (istream &is, vector<T>& V) {
    for(auto &e : V)
        is >> e;
    return is;
}
template<class T, size_t N> istream& operator >> (istream &is, array<T, N>& V) {
    for(auto &e : V)
        is >> e;
    return is;
}
#ifdef __SIZEOF_INT128__
ostream& operator << (ostream &os, __int128 const& value){
    static char buffer[64];
    int index = 0;
    __uint128_t T = (value < 0) ? (-(value + 1)) + __uint128_t(1) : value;
    if (value < 0) 
        os << '-';
    else if (T == 0)
        return os << '0';
    for(; T > 0; ++index){
        buffer[index] = static_cast<char>('0' + (T % 10));
        T /= 10;
    }    
    while(index > 0)
        os << buffer[--index];
    return os;
}
istream& operator >> (istream& is, __int128& T){
    static char buffer[64];
    is >> buffer;
    size_t len = strlen(buffer), index = 0;
    T = 0; int mul = 1;
    if (buffer[index] == '-')
        ++index, mul *= -1;
    for(; index < len; ++index)
        T = T * 10 + static_cast<int>(buffer[index] - '0');
    T *= mul;    
    return is;
}
#endif
template<typename CharT, typename Traits, typename T>
ostream& _containerprint(std::basic_ostream<CharT, Traits> &out, T const &val) {
    return (out << val << " ");
}
template<typename CharT, typename Traits, typename T1, typename T2>
ostream& _containerprint(std::basic_ostream<CharT, Traits> &out, pair<T1, T2> const &val) {
    return (out << "(" << val.first << "," << val.second << ") ");
}
template<typename CharT, typename Traits, template<typename, typename...> class TT, typename... Args>
ostream& operator << (std::basic_ostream<CharT, Traits> &out, TT<Args...> const &cont) {
    out << "[ ";
    for(auto&& elem : cont) _containerprint(out, elem);
    return (out << "]");
}
template<class L, class R> ostream& operator << (ostream& out, pair<L, R> const &val){
    return (out << "(" << val.first << "," << val.second << ") ");
}
template<typename L, size_t N> ostream& operator << (ostream& out, array<L, N> const &cont){
    out << "[ ";
    for(auto&& elem : cont) _containerprint(out, elem);
    return (out << "]");    
}
template<class T> ostream& operator<<(ostream &out, ordered_set<T> const& S){
    out << "{ ";
    for(const auto& s:S) out << s << " ";
    return (out << "}");
}
template<class L, class R, class chash = std::hash<L> > ostream& operator << (ostream &out, gp_hash_table<L, R, chash> const& M) {
    out << "{ ";
    for(const auto& m: M) out << "(" << m.first << ":" << m.second << ") ";
    return (out << "}");
}
template<class P, class Q = vector<P>, class R = less<P> > ostream& operator << (ostream& out, priority_queue<P, Q, R> const& M){
    static priority_queue<P, Q, R> U;
    U = M;
    out << "{ ";
    while(!U.empty())
        out << U.top() << " ", U.pop();
    return (out << "}");
}
template<class P> ostream& operator << (ostream& out, queue<P> const& M){
    static queue<P> U;
    U = M;
    out << "{ ";
    while(!U.empty())
        out << U.front() << " ", U.pop();
    return (out << "}");
}
#define TRACE
#ifdef TRACE
    #define trace(...) __f(#__VA_ARGS__, __VA_ARGS__)
    template <typename Arg1>
    void __f(const char* name, Arg1&& arg1){
        cerr << name << " : " << arg1 << endl;
    }
    template <typename Arg1, typename... Args>
    void __f(const char* names, Arg1&& arg1, Args&&... args){
        const char* comma = strchr(names + 1, ',');
        cerr.write(names, comma - names) << " : " << arg1<<" | ";
        __f(comma+1, args...);
    }
#else
    #define trace(...) 1
#endif

/*/---------------------------RNG----------------------/*/
mt19937_64 rng(chrono::steady_clock::now().time_since_epoch().count());
inline int64_t random_long(long long l = LLONG_MIN,long long r = LLONG_MAX){
    uniform_int_distribution<int64_t> generator(l,r);
    return generator(rng);
}
struct custom_hash { // Credits: https://codeforces.com/blog/entry/62393
    static uint64_t splitmix64(uint64_t x) {
        // http://xorshift.di.unimi.it/splitmix64.c
        x += 0x9e3779b97f4a7c15;
        x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
        x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
        return x ^ (x >> 31);
    }
    size_t operator()(uint64_t x) const {
        static const uint64_t FIXED_RANDOM = chrono::steady_clock::now().time_since_epoch().count();
        return splitmix64(x + FIXED_RANDOM);
    }
    template<typename L, typename R>
    size_t operator()(pair<L,R> const& Y) const{
        static const uint64_t FIXED_RANDOM = chrono::steady_clock::now().time_since_epoch().count();
        return splitmix64(Y.first * 31ull + Y.second + FIXED_RANDOM);
    }
};

/*/---------------------------Defines----------------------/*/
#define ll long long
#define endl "\n"
#define all(v) (v).begin(),(v).end()
/*/-----------------------Modular Arithmetic---------------/*/
const int mod = 1e9 + 7;
template<const int MOD>
struct modular_int{
	int x;
	static vector<int> inverse_list ;
	const static int inverse_limit;
	const static bool is_prime;
	modular_int(){
		x = 0;
	}
	template<typename T>  
	modular_int(const T z){
		x = (z%MOD);
		if (x < 0) x += MOD; 	
	}
	modular_int(const modular_int<MOD>* z){
		x = z->x;
	}
	modular_int(const modular_int<MOD>& z){
		x = z.x;
	}
	modular_int operator - (const modular_int<MOD>& m) const{
		modular_int<MOD> U;
		U.x = x - m.x;
		if (U.x < 0) U.x += MOD;
		return U;
	}
	modular_int operator + (const modular_int<MOD>& m) const{
		modular_int<MOD> U;
		U.x = x + m.x;
		if (U.x >= MOD) U.x -= MOD;
		return U;
	}
	modular_int& operator -= (const modular_int<MOD>& m){
		x -= m.x;
		if (x < 0) x += MOD;
		return *this;
	}	
	modular_int& operator += (const modular_int<MOD>& m){
		x += m.x;
		if (x >= MOD) x -= MOD;
		return *this;
	}	
	modular_int operator * (const modular_int<MOD>& m) const{
		modular_int<MOD> U;
		U.x = (x* 1ull * m.x) %MOD;
		return U;
	}
	modular_int& operator *= (const modular_int<MOD>& m){
		x = (x * 1ull * m.x)%MOD;
		return *this;
	}
	template<typename T>
	friend modular_int operator + (const T &l, const modular_int<MOD>& p){
		return (modular_int<MOD>(l) + p);
	}
	template<typename T>
	friend modular_int operator - (const T &l, const modular_int<MOD>& p){
		return (modular_int<MOD>(l) - p);
	}
	template<typename T>
	friend modular_int operator * (const T &l, const modular_int<MOD>& p){
		return (modular_int<MOD>(l) * p);
	}
	template<typename T>
	friend modular_int operator / (const T &l, const modular_int<MOD>& p){
		return (modular_int<MOD>(l) / p);
	}

	int value() const{
		return x;
	}

	modular_int operator ^ (const modular_int<MOD>& cpower) const{
		modular_int<MOD> ans;
		ans.x = 1;
		modular_int<MOD> curr(this);
		int power = cpower.x;

		if (curr.x <= 1){
			if (power == 0) curr.x = 1;
			return curr;
		}
		while( power > 0){
			if (power&1){
				ans *= curr;
			}
			power >>= 1;
			if (power) curr *= curr;
		}
		return ans;
	}


	modular_int operator ^ (long long power) const{
		modular_int<MOD> ans;
		ans.x = 1;
		modular_int<MOD> curr(this);
		if (curr.x <= 1){
			if (power == 0) curr.x = 1;
			return curr;
		}
		// Prime Mods
		if (power >= MOD && is_prime){
			power %= (MOD - 1);
		}

		while( power > 0){
			if (power&1){
				ans *= curr;
			}
			power >>= 1;
			if (power) curr *= curr;

		}
		return ans;
	}

	modular_int operator ^ (int power) const{
		modular_int<MOD> ans;
		ans.x = 1;
		modular_int<MOD> curr(this);
		
		if (curr.x <= 1){
			if (power == 0) curr.x = 1;
			return curr;
		}
		while( power > 0){
			if (power&1){
				ans *= curr;
			}
			power >>= 1;
			if (power) curr *= curr;

		}
		return ans;
	}
	
	template<typename T>
	modular_int& operator ^= (T power) {
		modular_int<MOD> res = (*this)^power;
		x = res.x;
		return *this;
	}


	template<typename T>
	modular_int pow(T x){
		return (*this)^x;
	}

	
	pair<long long,long long> gcd(const int a, const int b) const {
		if (b==0) return {1,0}; 
		pair<long long,long long> c = gcd(b , a%b);
		return { c.second , c.first - (a/b)*c.second}; 
	}

	inline void init_inverse_list() const {
		
		vector<int>& dp = modular_int<MOD>::inverse_list;
		dp.resize(modular_int<MOD>::inverse_limit + 1);
		int n = modular_int<MOD>::inverse_limit;
		dp[0] = 1;
		if (n > 1) dp[1] = 1;
		for(int i = 2; i <= n; ++i){
			dp[i] = (dp[MOD%i] * 1ull *(MOD - MOD/i))%MOD;
		}
	
	}
	modular_int<MOD> get_inv() const {
		if (modular_int<MOD>::inverse_list.size() < modular_int<MOD>::inverse_limit + 1) init_inverse_list();

		if (this->x <= modular_int<MOD>::inverse_limit){
			return modular_int<MOD>::inverse_list[this->x];
		}
		pair<long long,long long> G = gcd(this->x,MOD);
		return modular_int<MOD>(G.first);
	}
	modular_int<MOD> operator - () const {
		modular_int<MOD> v(0);
		v -= (*this);
		return v;
	}
	modular_int operator / (const modular_int<MOD>& m) const{
		modular_int<MOD> U(this);
		U *= m.get_inv();
		return U;
	}
	modular_int& operator /= (const modular_int<MOD>& m){
		(*this) *= m.get_inv();
		return *this;
	}
	bool operator==(const modular_int<MOD>& m) const{
		return x == m.x;
	}

	bool operator < (const modular_int<MOD>& m) const {
		return x < m.x;
	}


	template<typename T>
	bool operator == (const T& m) const{
		return (*this)== (modular_int<MOD>(m));
	}

	template<typename T>
	bool operator < (const T& m) const {
		return x < (modular_int<MOD>(m)).x;
	}
	template<typename T>
	bool operator > (const T& m) const {
		return x > (modular_int<MOD>(m)).x;
	}
	template<typename T>
	friend bool operator == (const T& x, const modular_int<MOD>& m) {
		return (modular_int<MOD>(x)).x == m.x;
	}
	template<typename T> 
	friend bool operator < (const T& x, const modular_int<MOD>& m){
		return (modular_int<MOD>(x)).x < m.x;
	}
	template<typename T> 
	friend bool operator > (const T& x, const modular_int<MOD>& m){
		return (modular_int<MOD>(x)).x > m.x;
	}

	friend istream& operator >> (istream& is, modular_int<MOD>& p){
		int64_t val;
		is >> val;
		p.x = (val%MOD);
		if (p.x < 0) p.x += MOD;
		return is;
	}
	friend ostream& operator << (ostream& os, const modular_int<MOD>& p) {return os<<p.x;}



};



using mint = modular_int<mod>;
template<const int MOD>
vector<int> modular_int<MOD>::inverse_list ;
template<const int MOD>
const int modular_int<MOD>::inverse_limit = -1;
template<const int MOD>
const bool modular_int<MOD>::is_prime = true;
// template<>   //-> useful if computing inverse fact = i_f[i-1] / i;
// const int modular_int<mod>::inverse_limit = 1000005;


/*/-----------------------------Code begins----------------------------------/*/
template<typename T, const int N, const int M>
struct Matrix {
	T arr[N][M];
	static T* c;
	T* operator [](int x) {
		return arr[x];
	}
    const T* operator [](int x) const {
    	return arr[x];
    }
	Matrix(){
		memset(arr, 0, sizeof(arr));
	}
	
	Matrix(const vector<vector<T>> & A){
		assert(A.size() == N && A[0].size() == M);
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j) 
				arr[i][j] = A[i][j];
	}

	Matrix(const T A[][M]){
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j) 
				arr[i][j] = A[i][j];
	}

	Matrix(const Matrix<T,N,M>& A){
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j) 
				arr[i][j] = A.arr[i][j];
	}

	inline void clear(){
		memset(arr, 0, sizeof(arr));
	}

	inline void make_identity(){
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j) 
				arr[i][j] = (i == j) ? 1 : 0;
	}

	Matrix operator + (const Matrix<T,N,M>& m) const {
		Matrix<T,N,M> ans;

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				ans.arr[i][j] = arr[i][j] + m.arr[i][j];

		return ans;
	}

	Matrix operator - (const Matrix<T,N,M>& m) const {
		Matrix<T,N,M> ans;

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				ans.arr[i][j] = arr[i][j] - m.arr[i][j];

		return ans;
	}

	template<const int L>
	Matrix<T,N,L> operator * (const Matrix<T,M,L>& m) const {
		Matrix<T,N,L> ans;

		for(int i = 0; i < N; ++i)
			for(int k = 0; k < M; ++k) {
				if (arr[i][k] == 0)
						continue;
				for(int j = 0; j < L; ++j)
					ans.arr[i][j] += arr[i][k] * m.arr[k][j];
			}

		return ans;
	}

	Matrix& operator += (const Matrix<T,N,M>& m) {
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				arr[i][j] += m.arr[i][j];
		return *this;
	}

	Matrix& operator -= (const Matrix<T,N,M>& m) {
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				arr[i][j] -= m.arr[i][j];
		return *this;
	}

	Matrix& operator *= (const Matrix<T,M,M>& m) {
		memset(c,0,sizeof(T)*N*M);
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j) {
				if (arr[i][j] == 0)
					continue;
				for(int k = 0; k < M; ++k)
					c[i * M + k] += arr[i][j] * m.arr[j][k];
			}

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				arr[i][j] = c[i * M + j];
		return *this;
	}

	Matrix<T,M,N> transpose() const {
		Matrix<T,M,N> ans;

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				ans.arr[j][i] = arr[i][j];
		return ans; 
	}

	void self_transpose(){
		static_assert( N == M );
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < i; ++j)
				swap(arr[i][j],arr[j][i]);
	}


	vector<T> operator *(const vector<T>& A) const{
		assert(A.size() == M);

		vector<T> ans(N,0);

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)

				ans[i] += arr[i][j] * A[j];

		return ans;
	}

	template<typename P>
	Matrix operator ^ (P power) const {
		static_assert( N == M );
		Matrix<T,N,M> ans, multiplier(*this);
		if (power == 1)
			return multiplier;
		ans.make_identity();

		while( power > 0 ){
			if ( power & 1 )
				ans *= multiplier;

			power >>= 1;
			if (power > 0) 
				multiplier *= multiplier;
		}

		return ans;
	}

	template<typename P>
	Matrix& operator ^= (P power) {
		static_assert( N == M );

		if (power == 1)
			return *this;
		Matrix<T,N,M> multiplier(*this);
		make_identity();

		while( power > 0 ){
			if ( power & 1 )
				(*this) *= multiplier;

			power >>= 1;
			if (power > 0) 
				multiplier *= multiplier;
		}

		return *this;
	}



	bool operator == (const Matrix other) const{
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				if (arr[i][j] != other.arr[i][j]) 
					return false;

		return true;
	}

	bool operator != (const Matrix other) const{
		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				if (arr[i][j] == other.arr[i][j]) 
					return false;

		return true;
	}


	friend vector<T> operator * (const vector<T>& A, const Matrix<T,N,M>& p) {
		assert(A.size() == N);

		vector<T> ans(M,0);

		for(int i = 0; i < N; ++i)
			for(int j = 0; j < M; ++j)
				ans[j] += A[i] * p.arr[i][j];

		return ans;
	}
	friend ostream& operator << (ostream& os, const Matrix<T,N,M> & p) {
		for(int i = 0; i < N; ++i){
			for(int j = 0; j < M; ++j){
				os << p.arr[i][j] << " ";
			}
			os << '\n';
		}
		return os;
	}
	
};

template<typename T,const int N , const int M> 
T* Matrix<T,N,M>::c = new T[N*M]; // required for *= operator

template<typename T, const int N>
using Square_Matrix = Matrix<T,N,N> ;


const int N = 2e5 + 100;
const int M = 20;

int P[N][M];
Square_Matrix<mint, 2> dp[N][M];
mint prob[N];
vector<int> adj[N];
int dep[N];
void dfs(int n, int p){
	P[n][0] = p;
	dep[n] = dep[p] + 1;
	for(auto e: adj[n]){
		if (e != p){
			vector<mint> tmp = {1 - prob[n], prob[n]};
			prob[e] = (dp[e][0] * tmp)[1];
			dfs(e, n);
		}
	}
}

void solve(){
    mint denominator = mint(1) / mint(1000000);

    int n, q;
    cin >> n >> q;

    mint K;
    cin >> K; K *= denominator;
    dp[1][0].make_identity();
    prob[1] = K;

    for(int i = 1; i <= n; ++i){
    	adj[i].clear();
    }
    for(int i = 2; i <= n; ++i){
    	mint A, B;
    	cin >> P[i][0] >> A >> B; A *= denominator; B *= denominator;
    	dp[i][0] = vector<vector<mint>>{{1 - B, 1 - A}, {B, A}};
    	adj[P[i][0]].push_back(i);

    }
    dep[1] = 0;
    dfs(1, 1);

    for(int i = 1; i < M; ++i){
    	for(int j = 1; j <= n; ++j){
    		P[j][i] = P[P[j][i - 1]][i - 1];
    		dp[j][i] = dp[j][i - 1] * dp[P[j][i - 1]][i - 1];
    	}
    }


    auto calc = [&] (int u, int v){
    	if (u == v){
    		return prob[u];
    	}
    	if (dep[u] > dep[v]){
    		swap(u, v);
    	}
    	Square_Matrix<mint, 2> a, b;
    	a.make_identity(); b.make_identity();

    	for(int i = 0; i < M; ++i){
    		if ((dep[v] - dep[u]) & (1 << i)){
    			b *= dp[v][i];
    			v = P[v][i];
    		}
    	}
    	if (v != u){
    		for(int j = M - 1; j >= 0; --j){
    			if (P[u][j] != P[v][j]){
    				a *= dp[u][j];
    				b *= dp[v][j];
    				u = P[u][j];
    				v = P[v][j];
    			}
    		}
    		a *= dp[u][0];
    		b *= dp[v][0];
    		u = P[u][0];
    		v = P[v][0];
    	}
    	assert (u == v);
    	mint q = 0;
    	vector<mint> cur = {1 - prob[u], prob[u]};
    	for(int i = 0; i < 2; ++i){
    		q += cur[i] * a[1][i] * b[1][i];
    	}
    	return q;

    };

    for(int jj = 1; jj <= q; ++jj){
    	int u, v;

    	cin >> u >> v;

    	cout << calc(u, v) << " ";
    }
    cout << endl;

}
int main(){
    // Use "set_name".max_load_factor(0.25);"set_name".reserve(512); with unordered set
    // Or use gp_hash_table<X,null_type>
    ios_base::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
    cout << fixed << setprecision(25);
    cerr << fixed << setprecision(10);
    auto start = std::chrono::high_resolution_clock::now();

    int t = 1;
    cin >> t;
    for(int i = 1; i <= t; ++i) {
    	cout << "Case #" << i << ": ";
        solve();
    }
    auto stop = std::chrono::high_resolution_clock::now(); 
    auto duration = std::chrono::duration_cast<std::chrono::nanoseconds>(stop - start);
    // cerr << "Time taken : " << ((long double)duration.count())/((long double) 1e9) <<"s "<< endl;     
}
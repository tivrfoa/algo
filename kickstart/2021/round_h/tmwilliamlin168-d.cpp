#include <bits/stdc++.h>
using namespace std;

#define ll long long
#define ld long double
#define ar array

#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace __gnu_pbds;

template <typename T> using oset = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;

#define vt vector
#define pb push_back
#define all(c) (c).begin(), (c).end()
#define sz(x) (int)(x).size()

#define F_OR(i, a, b, s) for (int i=(a); (s)>0?i<(b):i>(b); i+=(s))
#define F_OR1(e) F_OR(i, 0, e, 1)
#define F_OR2(i, e) F_OR(i, 0, e, 1)
#define F_OR3(i, b, e) F_OR(i, b, e, 1)
#define F_OR4(i, b, e, s) F_OR(i, b, e, s)
#define GET5(a, b, c, d, e, ...) e
#define F_ORC(...) GET5(__VA_ARGS__, F_OR4, F_OR3, F_OR2, F_OR1)
#define FOR(...) F_ORC(__VA_ARGS__)(__VA_ARGS__)
#define EACH(x, a) for (auto& x: a)

template<class T> bool umin(T& a, const T& b) {
	return b<a?a=b, 1:0;
}
template<class T> bool umax(T& a, const T& b) { 
	return a<b?a=b, 1:0;
} 

ll FIRSTTRUE(function<bool(ll)> f, ll lb, ll rb) {
	while(lb<rb) {
		ll mb=(lb+rb)/2;
		f(mb)?rb=mb:lb=mb+1; 
	} 
	return lb;
}
ll LASTTRUE(function<bool(ll)> f, ll lb, ll rb) {
	while(lb<rb) {
		ll mb=(lb+rb+1)/2;
		f(mb)?lb=mb:rb=mb-1; 
	} 
	return lb;
}

template<class A> void read(vt<A>& v);
template<class A, size_t S> void read(ar<A, S>& a);
template<class T> void read(T& x) {
	cin >> x;
}
void read(double& d) {
	string t;
	read(t);
	d=stod(t);
}
void read(long double& d) {
	string t;
	read(t);
	d=stold(t);
}
template<class H, class... T> void read(H& h, T&... t) {
	read(h);
	read(t...);
}
template<class A> void read(vt<A>& x) {
	EACH(a, x)
		read(a);
}
template<class A, size_t S> void read(array<A, S>& x) {
	EACH(a, x)
		read(a);
}

string to_string(char c) {
	return string(1, c);
}
string to_string(bool b) {
	return b?"true":"false";
}
string to_string(const char* s) {
	return string(s);
}
string to_string(string s) {
	return s;
}
string to_string(vt<bool> v) {
	string res;
	FOR(sz(v))
		res+=char('0'+v[i]);
	return res;
}

template<size_t S> string to_string(bitset<S> b) {
	string res;
	FOR(S)
		res+=char('0'+b[i]);
	return res;
}
template<class T> string to_string(T v) {
    bool f=1;
    string res;
    EACH(x, v) {
		if(!f)
			res+=' ';
		f=0;
		res+=to_string(x);
	}
    return res;
}

template<class A> void write(A x) {
	cout << to_string(x);
}
template<class H, class... T> void write(const H& h, const T&... t) { 
	write(h);
	write(t...);
}
void print() {
	write("\n");
}
template<class H, class... T> void print(const H& h, const T&... t) { 
	write(h);
	if(sizeof...(t))
		write(' ');
	print(t...);
}

void DBG() {
	cerr << "]" << endl;
}
template<class H, class... T> void DBG(H h, T... t) {
	cerr << to_string(h);
	if(sizeof...(t))
		cerr << ", ";
	DBG(t...);
}
#ifdef _DEBUG
#define dbg(...) cerr << "LINE(" << __LINE__ << ") -> [" << #__VA_ARGS__ << "]: [", DBG(__VA_ARGS__)
#else
#define dbg(...) 0
#endif

template<class T> void offset(ll o, T& x) {
	x+=o;
}
template<class T> void offset(ll o, vt<T>& x) {
	EACH(a, x)
		offset(o, a);
}
template<class T, size_t S> void offset(ll o, ar<T, S>& x) {
	EACH(a, x)
		offset(o, a);
}

mt19937 mt_rng(chrono::steady_clock::now().time_since_epoch().count());
ll randint(ll a, ll b) {
	return uniform_int_distribution<ll>(a, b)(mt_rng);
}

template<class T, class U> void vti(vt<T> &v, U x, size_t n) {
	v=vt<T>(n, x);
}
template<class T, class U> void vti(vt<T> &v, U x, size_t n, size_t m...) {
	v=vt<T>(n);
	EACH(a, v)
		vti(a, x, m);
}

const int d4i[4]={-1, 0, 1, 0}, d4j[4]={0, 1, 0, -1};
const int d8i[8]={-1, -1, 0, 1, 1, 1, 0, -1}, d8j[8]={0, 1, 1, 1, 0, -1, -1, -1};

const int M=1e9+7, mxA=1e6;
ll iv[mxA+1], anc[200000][18], pa[200000][18], pb[200000][18];

void solve() {
	int n, q;
	read(n, q);
	vt<ll> pk(n);
	read(pk[0]);
	pk[0]=pk[0]*iv[mxA]%M;
	vt<int> pr(n), d(n);
	FOR(k, 18)
		anc[0][k]=-1;
	FOR(i, 1, n) {
		read(pr[i]);
		--pr[i];
		read(pa[i][0], pb[i][0]);
		pa[i][0]=pa[i][0]*iv[mxA]%M;
		pb[i][0]=pb[i][0]*iv[mxA]%M;
		pk[i]=(pk[pr[i]]*pa[i][0]+(M+1-pk[pr[i]])*pb[i][0])%M;
		anc[i][0]=pr[i];
		FOR(k, 1, 18) {
			anc[i][k]=~anc[i][k-1]?anc[anc[i][k-1]][k-1]:-1;
			if(~anc[i][k]) {
				pa[i][k]=(pa[i][k-1]*pa[anc[i][k-1]][k-1]+pb[i][k-1]*(M+1-pa[anc[i][k-1]][k-1]))%M;
				pb[i][k]=(pa[i][k-1]*pb[anc[i][k-1]][k-1]+pb[i][k-1]*(M+1-pb[anc[i][k-1]][k-1]))%M;
			}
		}
		d[i]=d[pr[i]]+1;
	}
	while(q--) {
		int u, v;
		read(u, v), --u, --v;
		if(d[u]<d[v])
			swap(u, v);
		auto agg=[](int &u, int k, ar<ll, 2> &a) {
			a={(a[0]*pa[u][k]+a[1]*(M+1-pa[u][k]))%M, (a[0]*pb[u][k]+a[1]*(M+1-pb[u][k]))%M};
			u=anc[u][k];
		};
		ar<ll, 2> au{1, 0}, av{1, 0};
		FOR(k, 17, -1, -1)
			if(d[u]-(1<<k)>=d[v])
				agg(u, k, au);
		if(u^v) {
			FOR(k, 17, -1, -1)
				if(anc[u][k]^anc[v][k]) {
					agg(u, k, au);
					agg(v, k, av);
				}
			agg(u, 0, au);
			agg(v, 0, av);
		}
		write((au[0]*av[0]%M*pk[u]+au[1]*av[1]%M*(M+1-pk[u]))%M);
		write(' ');
	}
	print();
}

int main() {
	ios::sync_with_stdio(0);
	cin.tie(0);

	iv[1]=1;
	FOR(i, 2, mxA+1)
		iv[i]=M-M/i*iv[M%i]%M;
	int t=1;
	read(t);
	FOR(t) {
		write("Case #", i+1, ": ");
		solve();
	}
}
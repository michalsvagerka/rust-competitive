use std::cmp::min;
use std::mem::swap;

pub struct SuffixArray<'a, T> {
    pub input: &'a [T],
    pub length: usize,

    /// Suffixes of input sorted lexicographically
    pub suffix: Vec<usize>,

    /// Index array of suffix -- i.e. `inverse[i]` is the lexicographical order of `suffix[i..]`
    pub inverse: Vec<usize>,
}

impl<'a, T: Ord> SuffixArray<'a, T> {
    pub fn new(input: &'a [T]) -> Self {
        let length = input.len();

        let mut ctx = SuffixArrayComputation {
            length,
            input: vec![0; length],
            counts: vec![0; length],
            suffix: vec![0; length],
            suffix_tmp: vec![0; length],
        };


        let mut char_sorted = input.iter().enumerate().map(|(a, b)| (b, a)).collect::<Vec<_>>();
        char_sorted.sort();
        let mut r = 0usize;
        for i in 1..length {
            r += (char_sorted[i - 1].0 != char_sorted[i].0) as usize;
            ctx.input[char_sorted[i].1] = r;
            ctx.suffix[char_sorted[i].1] = i;
        }

        let mut input_tmp = vec![0; length];

        let mut k = 1;
        while k < length {
            ctx.counting_sort(k, r + 1);
            ctx.counting_sort(0, r + 1);

            r = 0;
            input_tmp[ctx.suffix[0]] = 0;
            for i in 1..length {
                let si = ctx.suffix[i];
                let sip = ctx.suffix[i - 1];
                r += (ctx.input[si] != ctx.input[sip] || si + k >= length || sip + k >= length ||
                    ctx.input[si + k] != ctx.input[sip + k]) as usize;
                input_tmp[si] = r;
            }
            swap(&mut input_tmp, &mut ctx.input);
            if ctx.input[ctx.suffix[length - 1]] == length - 1 {
                // already sorted
                break;
            }

            k *= 2;
        }

        let mut reverse = vec![0; length];
        for i in 0..length {
            reverse[ctx.suffix[i]] = i;
        }
        Self {
            input,
            length,
            suffix: ctx.suffix,
            inverse: reverse,
        }
    }

    pub fn next(&self, i: usize) -> Option<&usize> {
        self.suffix.get(self.inverse[i] + 1)
    }

    pub fn compute_lcp(&self) -> LongestCommonPrefix {
        LongestCommonPrefix::new(&self)
    }
}

pub struct LongestCommonPrefix {
    /// `LCP[i]` is the longest prefix of `&input[i..]` with its successor.
    pub lcp: Vec<usize>,
}

impl LongestCommonPrefix {
    pub fn new<T: Eq>(suffix_array: &SuffixArray<T>) -> Self {
        Self {
            lcp: (0..suffix_array.length).scan(0, |k, i| {
                if suffix_array.inverse[i] == suffix_array.length - 1 {
                    *k = 0;
                    Some(0)
                } else {
                    // while i + k < suffix_array.length &&
                    //     suffix_array.suffix[suffix_array.inverse[i] + 1] + k < suffix_array.length &&
                    //     suffix_array.input[i + k] == suffix_array.input[suffix_array.suffix[suffix_array.inverse[i] + 1] + k] {
                    //     k += 1;
                    // }
                    // note: both of these cannot be None at the same time
                    let next = suffix_array.suffix[suffix_array.inverse[i] + 1];
                    while suffix_array.input.get(i + *k) == suffix_array.input.get(next + *k) {
                        *k += 1;
                    }
                    let lcp = *k;
                    if *k > 0 { *k -= 1; }
                    Some(lcp)
                }
            }).collect(),
        }
    }

    /// Use indexes to the original array. If you want to find LCP of the i-th lexicographically
    /// smallest string, use `lcp(suffix_array.suffix[i])` instead.
    pub fn lcp(&self, i: usize) -> usize {
        self.lcp[i]
    }
}

pub struct RangeLongestCommonPrefix {
    pub rmq: Vec<Vec<usize>>,
}

impl RangeLongestCommonPrefix {
    pub fn new<T: Eq>(suffix_array: &SuffixArray<T>, lcp: &LongestCommonPrefix) -> Self {
        let mut rmq = vec![];
        rmq.push(lcp.lcp.clone());
        let mut p = 0;
        while (1 << p) < suffix_array.length {
            rmq.push((0..(suffix_array.length - (1 << p))).map(|i|
                min(rmq[p][i], rmq[p][i + (1 << p)])
            ).collect());
            p += 1;
        }
        Self {
            rmq
        }
    }

    /// Use indexes to the original array. If you want to find LCP of the i-th lexicographically
    /// smallest string, use `lcp(suffix_array.suffix[i], suffix_array.suffix[j])` instead.
    pub fn lcp(&self, _i: usize, _j: usize) {}
}

struct SuffixArrayComputation {
    length: usize,
    input: Vec<usize>,
    counts: Vec<usize>,
    suffix: Vec<usize>,
    suffix_tmp: Vec<usize>,
}

impl SuffixArrayComputation {
    fn counting_sort(&mut self, k: usize, r: usize) {
        for x in &mut self.counts[0..r] {
            *x = 0
        }

        self.counts[0] += k;
        let mut sum = 0;
        for i in 0..r {
            sum += self.counts[i];
            self.counts[i] = sum - self.counts[i];
        }

        for &s in &self.suffix {
            let idx = self.input.get(s + k).copied().unwrap_or(0);
            self.counts[idx] += 1;
            self.suffix_tmp[self.counts[idx]] = s;
        }

        swap(&mut self.suffix_tmp, &mut self.suffix)
    }
}

// // ordinary suffix array with optional LCP and LCP RMQ. look elsewhere
// template<typename Index, bool PrecomputeLCP = false, bool PrecomputeRMQ = false>
// class SuffixArray {
// public:
// 	static_assert(PrecomputeLCP || !PrecomputeRMQ, "Must have RMQ for LCP");
//
// 	template<typename T>
// 	explicit SuffixArray(T t):N(t.size()), S(N), I(N), LCP(PrecomputeLCP ? N : 0) {
// 		typedef typename std::remove_reference<decltype(t[0])>::type Item;
// 		vector<pair<Item, Index>> TR(N);
// 		for (Index i = 0; i < N; ++i) { TR[i] = {t[i], i}; }
// 		sort(TR.begin(), TR.end());
// 		vector<Index> R(N);
// 		Index r = R[TR[0].y] = S[TR[0].y] = 0;
// 		for (Index i = 1; i < N; ++i) {
// 			R[TR[i].y] = r += (TR[i - 1].x != TR[i].x);
// 			S[TR[i].y] = i;
// 		}
// 		vector<Index> RA(N), SA(N), C(N);
// 		for (Index k = 1; k < N; k <<= 1) {
// 			counting_sort(R, C, SA, k, r+1);
// 			counting_sort(R, C, SA, 0, r+1);
// 			RA[S[0]] = r = 0;
// 			for (Index i = 1; i < N; ++i) {
// 				RA[S[i]] = r += (R[S[i]] != R[S[i-1]] || S[i]+k >= N || S[i-1]+k >= N ||
// 								R[S[i]+k] != R[S[i-1]+k]);
// 			}
// 			swap(RA, R);
// 			if (R[S[N-1]] == N-1) break;
// 		}
// 		for (Index i = 0; i < N; ++i) { I[S[i]] = i; }
// 		if (PrecomputeLCP) {
// 			Index k = 0;
// 			for (Index i = 0; i < N; ++i) {
// 				if (I[i] == N - 1) {
// 					LCP[I[i]] = k = 0;
// 					continue;
// 				}
// 				while (i + k < N && S[I[i] + 1] + k < N && t[i + k] == t[S[I[i] + 1] + k]) { ++k; }
// 				LCP[I[i]] = k;
// 				if (k > 0) { --k; }
// 			}
// 		}
// 		if (PrecomputeRMQ) {
// 			RMQ.push_back(LCP);
// 			for (int p = 0; (1 << p) < N; ++p) {
// 				RMQ.push_back(RMQ[p]);
// 				for (int i = 0; i < N - (1 << p); ++i) { RMQ[p + 1][i] = min(RMQ[p + 1][i], RMQ[p][i + (1 << p)]); }
// 			}
// 		}
// 	}
//
// 	template<typename=std::enable_if<PrecomputeLCP>>
// 	Index lcp(Index i) const { return LCP[I[i]]; }
//
// 	template<typename=std::enable_if<PrecomputeRMQ>>
// 	Index lcp(Index i, Index j) const {
// 		i = I[i];
// 		j = I[j];
// 		if (i > j) { swap(i, j); }
// 		if (i == j - 1)return LCP[i];
// 		Index p = 0;
// 		while ((1 << p) < j - i) { ++p; }
// 		--p;
// 		return min(RMQ[p][i], RMQ[p][j - (1 << p)]);
// 	}


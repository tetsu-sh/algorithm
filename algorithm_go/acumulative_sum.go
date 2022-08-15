package main

import (
	"fmt"
	"math"
)

func guest_sum() {
	n := []int{3, 4, 18, 3, 18, 48, 29, 19, 23, 42}
	l := []int{1, 2, 3, 5, 1, 2, 3, 5, 8, 6, 4}
	r := []int{2, 8, 7, 9, 4, 5, 4, 9, 9, 7, 8}
	s := make([]int, len(n), len(n))
	for i := 0; i < len(n)-1; i++ {
		s[i+1] = s[i] + n[i]
	}
	for i := 0; i < len(l)-1; i++ {
		ans := s[r[i]] - s[l[i]]
		println(ans)
	}
}

func train_distance() {
	a := []int{100, 400, 483, 413, 413, 471, 784}
	b := []int{1, 4, 3, 5, 2, 6}
	s := make([]int, len(a), len(a))
	s[0] = a[0]
	for i := 1; i < len(a); i++ {
		s[i] = s[i-1] + a[i]
	}
	ans := 0.
	for i := 0; i < len(b)-1; i++ {

		k := s[b[i+1]] - s[b[i]]
		ans += math.Abs(float64(k))
	}
	fmt.Println(ans)

}

func count_worker() {
	t := 18
	ls := []int{4, 7, 7, 12, 13, 16}
	rs := []int{8, 12, 13, 15, 17, 18}
	s := make([]int, t+1, t+1)
	for i := 0; i < len(ls); i++ {
		s[ls[i]] += 1
		s[rs[i]] -= 1
	}
	fmt.Println(s)
	ans := 0
	for i := 0; i <= t; i++ {
		ans += s[i]
		fmt.Printf("%dji %dniniru\n", i, ans)
	}

}

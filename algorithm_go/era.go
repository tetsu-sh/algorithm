package main

import (
	"fmt"
	"math"
)

func print_prime_numbers(num int) int {
	// var num int
	// fmt.Scan(&num)
	prime := make([]bool, num+1, num+1)
	for i := 2; i <= num; i++ {
		prime[i] = true
	}
	for i := 2; i*i <= num; i++ {
		if prime[i] == true {
			for x := 2 * i; x <= num; x += i {
				prime[x] = false
			}
		}
	}
	count := 0
	for i := 2; i <= num; i++ {
		if prime[i] == true {
			// fmt.Printf("%d ", i)
			count += 1
		}
	}
	return count

}

func kubunkyuuseki() {
	var ans float64 = 0.
	n := 100
	for i := 0; i < n; i++ {
		x := (float64(i) / float64(n))
		ans += math.Pow(2, x*x)
	}
	fmt.Printf("kotaeha:%g\n", ans/float64(n))
}

func sum_divisors() {
	// n := 100
	var n int
	fmt.Scan(&n)
	l := make([]int, n+1, n+1)
	for i := 1; i <= n; i++ {
		for j := i; j <= n; j += i {
			l[j] += 1
		}
	}
	ans := 0
	for i, n := range l {
		ans += i * n
	}
	fmt.Printf("%d\n", ans)
}

func over_num_by_sum_inverse() {
	n := 0
	goal := 4
	sum := 0
	for i := 1; sum < goal; i++ {
		n += 1
		sum += 1 / i
	}
	fmt.Printf("%d\n", n)

}

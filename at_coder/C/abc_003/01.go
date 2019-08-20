package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	var k int
	fmt.Scan(&n)
	fmt.Scan(&k)

	rs := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&rs[i])
	}

	sort.Sort(sort.Reverse(sort.IntSlice(rs)))

	var v = 2.
	var ans = 0.
	for i := 0; i < k; i++ {
		ans += float64(rs[i]) / v
		v *= 2
	}
	fmt.Printf("%v\n", ans)
}

package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)
	a := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Scan(&a[i])
	}

	var ans = 0. //float64(n)
	for i := 0; i < n; i++ {
		var cnt = 0
		for j := 0; j < n; j++ {
			if a[i]%a[j] == 0 {
				cnt++
			}
		}
		ans += map[bool]float64{true: 0.5, false: float64(cnt+1) / float64(2*cnt)}[cnt%2 == 0]
	}
	fmt.Println(ans)
}

package main

import (
	"fmt"
)

/*
func f(s []int) bool {
	for i := 0; i < 6; i++ {
		if s[i] != i+1 {
			return false
		}
	}
	return true
}
*/

func main() {
	var n int
	fmt.Scan(&n)

	s := make([]int, 6)
	for i := 0; i < 6; i++ {
		s[i] = i + 1
	}
	for i := 0; i < n%30; i++ {
		tmp := s[i%5]
		s[i%5] = s[i%5+1]
		s[i%5+1] = tmp
		/*
			if f(s) {
				fmt.Println(i)
			}
		*/
	}
	for i := 0; i < 6; i++ {
		fmt.Print(s[i])
	}
	fmt.Println()
}

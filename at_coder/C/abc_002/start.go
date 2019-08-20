package main

import (
	"fmt"
	"math"
)

func main() {
	var xa int
	var ya int
	var xb int
	var yb int
	var xc int
	var yc int

	fmt.Scan(&xa)
	fmt.Scan(&ya)
	fmt.Scan(&xb)
	fmt.Scan(&yb)
	fmt.Scan(&xc)
	fmt.Scan(&yc)

	var a = xb - xa
	var b = yb - ya

	var c = xc - xa
	var d = yc - ya

	fmt.Printf("%v", math.Abs(float64(a*d-b*c)/2.0))
}

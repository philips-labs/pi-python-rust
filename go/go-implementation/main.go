package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	pi_est := montecarlo_pi(5000000)
	fmt.Println(pi_est)
}

func montecarlo_pi(trials int) float64 {
	hit := 0
	s := rand.NewSource(time.Now().UnixNano())
	r := rand.New(s)

	for i := 0; i <= trials; i++ {
		x := r.Float64()
		y := r.Float64()

		pos_on_board := x*x + y*y

		if pos_on_board < 1.0 {
			hit = hit + 1
		}
	}

	return float64(hit) * 4.0 / float64(trials)
}

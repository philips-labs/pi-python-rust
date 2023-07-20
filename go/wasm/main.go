package main

import (
	"math/rand"
	"time"
	"syscall/js"
)

func main() {
  c := make(chan struct{}, 0)
  println("Go WebAssembly Initialized")

  js.Global().Set("monteCarloPi", js.FuncOf(monteCarloPi))

  <-c
}

func monteCarloPi(this js.Value, args []js.Value) interface{} {
  trials := args[0].Int()
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

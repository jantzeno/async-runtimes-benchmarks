package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"sync"
)

func main() {
	numRoutines := 100_000
	if len(os.Args) > 1 {
		n, err := strconv.Atoi(os.Args[1])
		if err == nil {
			numRoutines = n
		}
	}

	maxTemp := 100.0
	minTemp := 0.0
	var wg sync.WaitGroup
	var mutex sync.Mutex
	stat := &runningStat{}

	for i := 0; i < numRoutines; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mutex.Lock()
			stat.push(randFloat64(minTemp, maxTemp))
			mutex.Unlock()
		}()
	}

	wg.Wait()

	fmt.Println("All goroutines finished.")

	fmt.Printf("Mean: %.1f\n", stat.mean())
	fmt.Println("Count: ", stat.count)
}

func randFloat64(min, max float64) float64 {
	return rand.Float64()*(max-min) + min
}

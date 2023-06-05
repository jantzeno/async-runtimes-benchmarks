package main

import (
	"crypto/sha512"
	"fmt"
	"os"
	"strconv"
	"sync"

	"github.com/google/uuid"
)

func main() {
	numRoutines := 100_000
	if len(os.Args) > 1 {
		n, err := strconv.Atoi(os.Args[1])
		if err == nil {
			numRoutines = n
		}
	}

	var wg sync.WaitGroup
	var mutex sync.Mutex
	var hashes []string

	for i := 0; i < numRoutines; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mutex.Lock()
			// work
			uuid := uuid.New().String()
			hash := sha512.Sum512([]byte(uuid))
			hashes = append(hashes, fmt.Sprintf("%x", hash))
			mutex.Unlock()
		}()
	}

	wg.Wait()

	fmt.Println("All goroutines finished.")

	fmt.Println("Hash Count: ", len(hashes))
}

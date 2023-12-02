package utils

import (
	"bufio"
	"os"
)

func ReadFile(path string) []string {
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var input []string

	for scanner.Scan() {
		input = append(input, scanner.Text())
	}

	file.Close()
	return input
}

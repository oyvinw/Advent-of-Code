package utils

import (
	"bufio"
	"os"
)

// ReadFileLines reads the file line by line
func ReadFileLines(path string) []string {
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var output []string

	for scanner.Scan() {
		output = append(output, scanner.Text())
	}

	file.Close()
	return output
}

// ReadFileChars reads the file rune for rune
func ReadFileChars(path string) [][]rune {
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var output [][]rune

	for scanner.Scan() {
		var line []rune
		text := []rune(scanner.Text())

		for i := range text {
			line = append(line, text[i])
		}

		output = append(output, line)
	}

	file.Close()
	return output
}

// Abs No abs for ints in std math :)
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// Min or min hur dur
func Min(x, y int) int {
	if x > y {
		return y
	}
	return x
}

package utils

import (
	"bufio"
	"os"
)

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

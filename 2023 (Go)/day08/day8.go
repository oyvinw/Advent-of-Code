package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"strings"
)

type Node struct {
	Left  string
	Right string
}

func main() {
	data := utils.ReadFileLines("day8.txt")
	instruction := make([]int32, 0)
	startNodes := make([]string, 0)

	nodeMap := make(map[string]Node)

	for _, val := range data[0] {
		instruction = append(instruction, val)
	}

	for i := 2; i < len(data); i++ {
		line := data[i]
		split := strings.Split(line, "=")
		nodeName := strings.TrimSpace(split[0])
		leftRightSplit := strings.Split(split[1], ",")
		leftPath := strings.TrimPrefix(strings.TrimSpace(leftRightSplit[0]), "(")
		rightPath := strings.TrimSuffix(strings.TrimSpace(leftRightSplit[1]), ")")

		nodeMap[nodeName] = Node{
			Left:  leftPath,
			Right: rightPath,
		}

		if nodeName[len(nodeName)-1:] == "A" {
			startNodes = append(startNodes, nodeName)
		}
	}
	fmt.Println(startNodes)

	//P1
	iterations := 0
	currentNodeKey := "AAA"
	for currentNodeKey != "ZZZ" {
		switch instruction[iterations%len(instruction)] {
		case 'L':
			currentNodeKey = nodeMap[currentNodeKey].Left
		case 'R':
			currentNodeKey = nodeMap[currentNodeKey].Right
		}
		iterations++
	}
	fmt.Println("P1:", iterations)

	//P2
	currentNodeKeys := startNodes
	iterationTimes := make([]int, len(currentNodeKeys))

	for i := range currentNodeKeys {
		for !pathAtDestination(currentNodeKeys[i]) {
			switch instruction[iterationTimes[i]%len(instruction)] {
			case 'L':
				currentNodeKeys[i] = nodeMap[currentNodeKeys[i]].Left
			case 'R':
				currentNodeKeys[i] = nodeMap[currentNodeKeys[i]].Right
			}
			iterationTimes[i]++
		}
	}

	p2 := 1
	for _, val := range iterationTimes {
		p2 = p2 * val / GCD(p2, val)
	}

	fmt.Println("P2:", p2)
}

func pathAtDestination(node string) bool {
	if node[len(node)-1:] != "Z" {
		return false
	}
	return true
}

func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

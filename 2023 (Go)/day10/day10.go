package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
)

type Pos struct {
	x, y int
}

var data = utils.ReadFileChars("day10.txt")

func main() {
	startingPoint := findStartingPoint()
	startingConnections := IdentifyStart(startingPoint)

	loopArea := make([][]bool, len(data))
	for i := range loopArea {
		loopArea[i] = make([]bool, len(data[i]))
	}

	previousPos := startingPoint
	currentPos := startingConnections[0]
	loopArea[startingPoint.y][startingPoint.x] = true
	steps := 1
	for currentPos != startingPoint {
		steps++
		loopArea[currentPos.y][currentPos.x] = true
		currentPos, previousPos = FindNext(currentPos, previousPos)
	}

	area := 0
	for y := range loopArea {
		withinLoop := false
		for x := range loopArea[y] {
			if loopArea[y][x] {
				switch data[y][x] {
				case 'J', 'L', '|', 'S':
					withinLoop = !withinLoop
				}
			} else if withinLoop {
				area++
			}
		}
	}

	fmt.Println("P1:", steps/2)
	fmt.Println("P2:", area)
}

func FindNext(currentPos Pos, cameFrom Pos) (Pos, Pos) {
	pipe := data[currentPos.y][currentPos.x]
	direction := Pos{currentPos.x - cameFrom.x, currentPos.y - cameFrom.y}
	switch pipe {
	case '|':
		return Pos{currentPos.x, currentPos.y + direction.y}, currentPos
	case '-':
		return Pos{currentPos.x + direction.x, currentPos.y}, currentPos
	case 'L':
		if direction.x != 0 {
			return Pos{currentPos.x, currentPos.y - 1}, currentPos
		} else {
			return Pos{currentPos.x + 1, currentPos.y}, currentPos
		}
	case 'J':
		if direction.x != 0 {
			return Pos{currentPos.x, currentPos.y - 1}, currentPos
		} else {
			return Pos{currentPos.x - 1, currentPos.y}, currentPos
		}
	case '7':
		if direction.x != 0 {
			return Pos{currentPos.x, currentPos.y + 1}, currentPos
		} else {
			return Pos{currentPos.x - 1, currentPos.y}, currentPos
		}
	case 'F':
		if direction.x != 0 {
			return Pos{currentPos.x, currentPos.y + 1}, currentPos
		} else {
			return Pos{currentPos.x + 1, currentPos.y}, currentPos
		}
	}
	panic("loop terminated")
}

var neighbours = []Pos{{0, 1}, {-1, 0}, {1, 0}, {0, -1}}

func IdentifyStart(pos Pos) []Pos {
	connections := make([]Pos, 0)
	for i, n := range neighbours {
		nPos := Pos{pos.x + n.x, pos.y + n.y}
		if (nPos.x < 0 || nPos.x > len(data[0])-1) || (nPos.y < 0 || nPos.y > len(data)-1) {
			continue
		}

		if i == 0 {
			switch data[nPos.y][nPos.x] {
			case '|', 'L', 'J':
				connections = append(connections, nPos)
			}
		}
		if i == 1 {
			switch data[nPos.y][nPos.x] {
			case '-', 'L', 'F':
				connections = append(connections, nPos)
			}
		}
		if i == 2 {
			switch data[nPos.y][nPos.x] {
			case '-', 'J', '7':
				connections = append(connections, nPos)
			}
		}
		if i == 3 {
			switch data[nPos.y][nPos.x] {
			case '|', 'F', '7':
				connections = append(connections, nPos)
			}
		}
	}
	return connections
}

func findStartingPoint() Pos {
	for y := range data {
		for x := range data[y] {
			if data[y][x] == 'S' {
				return Pos{x, y}
			}
		}
	}
	return Pos{-1, -1}
}

package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
)

func main() {
	data := utils.ReadFileChars("data11.txt")
	fmt.Println(data)
}

func distance(ax, ay, bx, by int) int {
	yDist := utils.Abs(by - ay)
	xDist := utils.Abs(bx - ax)
	diags := utils.Min(yDist, xDist)
	return yDist + xDist - diags
}

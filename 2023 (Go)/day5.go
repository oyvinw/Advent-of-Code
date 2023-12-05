package main

import (
	"fmt"
	"github.com/oyvinw/Advent-of-Code/utils"
	"math"
	"strconv"
	"strings"
	"time"
)

type SeedMapEntry struct {
	DestinationStart int
	SourceStart      int
	Range            int
}

func (s SeedMapEntry) DestinationEnd() int {
	return s.DestinationStart + (s.Range - 1)
}

func (s SeedMapEntry) SourceEnd() int {
	return s.SourceStart + (s.Range - 1)
}

func (s SeedMapEntry) evaluate(i int) int {
	return (i - s.SourceStart) + s.DestinationStart
}

var farmingMaps = make([][]SeedMapEntry, 7)

func main() {
	data := utils.ReadFileLines("data/day5.txt")

	seeds := make([]int, 0)

	currentMapParseIndex := -1

	//Parse and create maps
	for i := 0; i < len(data); i++ {
		line := data[i]

		if i == 0 {
			seedsData := strings.Fields(strings.Split(line, ":")[1])
			for s := range seedsData {
				seed, _ := strconv.Atoi(seedsData[s])
				seeds = append(seeds, seed)
			}
			continue
		}

		if len(line) == 0 {
			currentMapParseIndex++
			i++
			continue
		}

		mappings := strings.Fields(line)

		destStart, _ := strconv.Atoi(mappings[0])
		srcStart, _ := strconv.Atoi(mappings[1])
		rangeLen, _ := strconv.Atoi(mappings[2])

		seedEntry := SeedMapEntry{
			DestinationStart: destStart,
			SourceStart:      srcStart,
			Range:            rangeLen,
		}

		farmingMaps[currentMapParseIndex] = append(farmingMaps[currentMapParseIndex], seedEntry)
	}

	lowest := math.MaxInt

	//P1
	for s := range seeds {
		val := evaluateSeedThroughMaps(seeds[s])
		if val < lowest {
			lowest = val
		}
	}

	fmt.Println("P1: ", lowest)
	lowest = math.MaxInt

	//P2 (Neanderthal implementation)
	/*
		for s := 0; s < len(seeds); s += 2 {
			for i := seeds[s]; i < seeds[s]+seeds[s+1]; i++ {
				val := evaluateSeedThroughMaps(i)
				if val < lowest {
					lowest = val
				}
			}
		}
	*/

	start := time.Now()

	//P2 (Cro-Magnon implementation)
	results := make(chan int)

	for s := 0; s < len(seeds); s += 2 {
		go func(s int) {
			thisLowest := math.MaxInt
			for i := seeds[s]; i < seeds[s]+seeds[s+1]; i++ {
				val := evaluateSeedThroughMaps(i)
				if val < thisLowest {
					thisLowest = val
				}
			}
			results <- thisLowest
			println("Goroutine", s/2, "finished in", time.Now().Sub(start).Milliseconds(), "ms")
		}(s)
	}

	for s := 0; s < len(seeds); s += 2 {
		r := <-results
		if r < lowest {
			lowest = r
		}
	}

	t := time.Now().Sub(start)
	println("P2 finished in", t.Milliseconds(), "ms. (", t.Minutes(), " minutes)")
	fmt.Println("P2: ", lowest)
}

func evaluateSeedThroughMaps(val int) int {
	for i := range farmingMaps {
		currentMap := farmingMaps[i]
		for j := range currentMap {
			currentMapEntry := currentMap[j]
			if val >= currentMapEntry.SourceStart && val <= currentMapEntry.SourceEnd() {
				val = currentMapEntry.evaluate(val)
				break
			}
		}
	}
	return val
}

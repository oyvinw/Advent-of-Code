﻿module day1

let calculateFuelForMass mass =
    (mass / 3) - 2

let rec calculateFuelForFuel mass =
    match mass with
    | _ when mass < 0 -> 0
    | m -> m + (calculateFuelForFuel (calculateFuelForMass m))

let data =
    System.IO.File.ReadLines("""C:\git\Advent-of-Code\2019 (F#)\solutions\day1\day1_input.txt""")

// part 1
data |> Seq.sumBy (fun x -> calculateFuelForMass(int x)) |> (printfn "part 1: %i")

// part 2
data |> Seq.sumBy (fun x -> calculateFuelForMass (int x) |> calculateFuelForFuel) |> (printfn "part 2: %i")

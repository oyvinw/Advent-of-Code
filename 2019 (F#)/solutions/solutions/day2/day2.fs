module day2

open System

let data: int array =
    System.IO.File.ReadAllText("""C:\git\Advent-of-Code\2019 (F#)\solutions\solutions\day2\day2_input.txt""")
    |> _.Split(',', StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map(int)
    |> Seq.toArray

let initialMemory = data
// let initialMemory = [|1;0;0;0;99|]
// let initialMemory = [|2;3;0;3;99|]
// let initialMemory = [|2;4;4;5;99;0|]

let getOperation (opcode : int): Option<int -> int -> int> =
    match opcode with
    | 1 -> Some(fun a b -> a + b)
    | 2 -> Some(fun a b -> a * b)
    | _ -> None

let rec calculateNext i (memory : int array) : int array =
    if i >= Array.length memory then
        memory
    else
        match getOperation memory[i] with
        | None -> memory
        | Some operation ->
        memory[memory[i + 3]] <- operation memory[memory[(i + 1)]] memory[memory[(i + 2)]]
        calculateNext (i + 4) memory

let setAndCalculate a b : int array =
    let localMem = Array.copy initialMemory
    localMem[1] <- a
    localMem[2] <- b
    calculateNext 0 localMem

// //part 1
setAndCalculate 12 2 |> Array.head |> (printfn "part 1: %i")

//part 2
let tupleOption =
    Array.allPairs [|for i in 0..99 -> i|] [|for i in 0..99 -> i|]
    |> Array.tryFind (fun (a, b) -> setAndCalculate a b |> Array.head = 19690720)

match tupleOption with
    | Some (noun, verb) -> printfn $"part 2: {100 * noun + verb}"
    | _ -> printfn "part 2: could not find an answer"



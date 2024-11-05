module day4

let lower = 128392
let higher = 643281

let inputSeq = seq {lower..higher}

let hasPairs sequence =
    sequence |> Seq.where (fun x -> (x.ToString() |> Seq.pairwise |> Seq.exists (fun (a, b) -> a = b)))

let isAscending sequence =
    sequence |> Seq.where (fun x -> (x.ToString() |> Seq.pairwise |> Seq.forall (fun (a, b) -> a <= b)))

let hasStandAlonePairs sequence =
    sequence |> Seq.where (fun number -> (let digits = number.ToString().ToCharArray()
    digits
    |> Seq.indexed
    |> Seq.exists (fun (i, value) ->
        (i + 1 < digits.Length && value = digits[i + 1]) && // Current digit equals the next digit
        (i + 2 >= digits.Length || value <> digits[i + 2]) && // Next digit is either out of bounds or not the same
        (i = 0 || value <> digits[i - 1]) // Previous digit is either out of bounds or not the same
    )))

let part1 =
    inputSeq |> hasPairs |> isAscending |> Seq.length |> (printfn "%i")

let part2 =
    inputSeq |> hasStandAlonePairs |> isAscending |> Seq.length |> (printfn "%i")
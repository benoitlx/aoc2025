let digits_of_line (s : string) : int list =
  s |> String.to_seq |> List.of_seq
  |> List.map (fun c -> Char.code c - Char.code '0')

let max_joltage_of_bank (digits : int list) : int =
  match digits with
  | _ :: _ :: _ ->
      let _, best =
        List.fold_right
          (fun x (max_right_opt, best) ->
            let best' =
              match max_right_opt with
              | None -> best
              | Some y -> max best ((10 * x) + y)
            in
            let max_right' =
              match max_right_opt with
              | None -> Some x
              | Some y -> Some (max y x)
            in
            (max_right', best'))
          digits (None, 0)
      in
      best
  | _ -> raise (Invalid_argument "bank must have at least two digits")

let lobby () =
  let rec loop acc =
    try
      let line = read_line () in
      let digits = digits_of_line line in
      loop (acc + max_joltage_of_bank digits)
    with _ -> acc
  in
  loop 0
;;

print_endline (string_of_int (lobby ()))

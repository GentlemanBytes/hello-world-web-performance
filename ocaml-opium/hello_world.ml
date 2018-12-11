open Opium.Std

let hello_world = get "/" begin fun _req ->
    `String ("Hello world!") |> respond'
end

let _ =
  App.empty
  |> hello_world
  |> App.run_command
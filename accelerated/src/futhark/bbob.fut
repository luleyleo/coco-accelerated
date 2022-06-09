import "raw"

module t = import "transform"

entry sphere (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    map2 (-) x xopt |> sphere_raw |> (+fopt)

entry ellipsoidal (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    map2 (-) x xopt |> t.Tosz |> ellipsoidal_raw |> (+fopt)

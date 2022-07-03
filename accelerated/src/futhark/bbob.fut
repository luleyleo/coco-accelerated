import "math"

module raw = import "raw"
module t = import "transform"

entry sphere (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt
    |> raw.sphere
    |> (+fopt)

entry ellipsoidal (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz
    |> raw.ellipsoidal
    |> (+fopt)

entry rastrigin (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz |> t.asy 0.2 |> t.A 10
    |> raw.rastrigin
    |> (+fopt)

local def bueche_rastrigin_s (x: []f64): []f64 =
    -- the next line checks for even indices because here indices start at zero.
    let s = map (\i -> if x[i] > 0 && (i % 2 == 0) then 10 else 1) (indices x) in
    map2 (*) s x

entry bueche_rastrigin (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz |> bueche_rastrigin_s |> t.A 10
    |> raw.rastrigin
    |> (+ 100 * (t.pen x))
    |> (+ fopt)

entry linear_slope (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    let z = map (\(xi, xopti) -> if xi * xopti < 25 then xi else xopti) (zip x xopt) in
    let s = t.A 100 (map sign' xopt) in
    zip s z
    |> map (\(si, zi) -> (5 * (f64.abs si) - si * zi))
    |> f64.sum
    |> (+ fopt)

entry attractive_sector (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> mat'vec Q |> t.A 10 |> mat'vec R
    |> zip xopt |> map (\(xopti, xi) -> if xi * xopti > 0 then 100 * xi else xi)
    |> raw.sphere
    |> t.y_osz
    |> (** 0.9)
    |> (+ fopt)

local def step_ellipsoidal_round (zi: f64): f64 =
    if (f64.abs zi) > 0.5
    then f64.floor(zi + 0.5)
    else f64.floor(zi * 10 + 0.5) / 10

entry step_ellipsoidal (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    let z' = x |> t.shift xopt |> mat'vec R |> t.A 10 in
    let z = x |> t.shift xopt |> mat'vec Q |> t.A 10 |> map step_ellipsoidal_round |> mat'vec R in
    z
    |> raw.step_ellipsoidal
    |> f64.max ((f64.abs z'[0]) / 10**4)
    |> (* 0.1)
    |> (+ fopt)
    |> (+ t.pen x)

entry rosenbrock (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt
    |> map (* (f64.max 1 ((f64.sqrt (dim x)) / 8)))
    |> map (+ 1)
    |> raw.rosenbrock
    |> (+ fopt)

entry rosenbrock_rotated (x: []f64) (R: [][]f64) (fopt: f64): f64 =
    x
    |> mat'vec R
    |> map (* (f64.max 1 ( (f64.sqrt (dim x)) / 8 )))
    |> map (+ 0.5)
    |> raw.rosenbrock
    |> (+ fopt)

entry ellipsoidal_rotated (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> mat'vec R |> t.x_osz
    |> raw.ellipsoidal
    |> (+fopt)

entry discus (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> mat'vec R |> t.x_osz
    |> raw.discus
    |> (+ fopt)

import "raw"
import "math"

module t = import "transform"

entry sphere (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt
    |> sphere_raw
    |> (+fopt)

entry ellipsoidal (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz
    |> ellipsoidal_raw
    |> (+fopt)

entry rastrigin (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz |> t.asy 0.2 |> t.A 10
    |> rastrigin_raw
    |> (+fopt)

local def bueche_rastrigin_s (x: []f64): []f64 =
    -- the next line checks for even indices because here indices start at zero.
    let s = map (\i -> if x[i] > 0 && (i % 2 == 0) then 10 else 1) (indices x) in
    map2 (*) s x

entry bueche_rastrigin (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    x
    |> t.shift xopt |> t.x_osz |> bueche_rastrigin_s |> t.A 10
    |> rastrigin_raw
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
    |> sphere_raw
    |> t.y_osz
    |> (** 0.9)
    |> (+ fopt)

entry ellipsoidal_rotated (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> mat'vec R |> t.x_osz
    |> ellipsoidal_raw
    |> (+fopt)

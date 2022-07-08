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
    |> t.shift xopt |> t.rotate Q |> t.A 10 |> t.rotate R
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
    let z' = x |> t.shift xopt |> t.rotate R |> t.A 10 in
    let z = x |> t.shift xopt |> t.rotate Q |> t.A 10 |> map step_ellipsoidal_round |> t.rotate R in
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

entry rosenbrock_rotated (x: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.rotate R
    |> map (* (f64.max 1 ( (f64.sqrt (dim x)) / 8 )))
    |> map (+ 0.5)
    |> raw.rosenbrock
    |> (+ fopt)

entry ellipsoidal_rotated (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.x_osz
    |> raw.ellipsoidal
    |> (+fopt)

entry discus (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.x_osz
    |> raw.discus
    |> (+ fopt)

entry bent_cigar (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.asy 0.5 |> t.rotate R
    |> raw.bent_cigar
    |> (+ fopt)

entry sharp_ridge (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate Q |> t.A 10 |> t.rotate R
    |> raw.sharp_ridge
    |> (+ fopt)

entry different_powers (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R
    |> raw.different_powers
    |> (+ fopt)

entry rastrigin_rotated (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.x_osz
    |> t.asy 0.2 |> t.rotate Q |> t.A 10 |> t.rotate R
    |> raw.rastrigin
    |> (+ fopt)

entry weierstrass (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R
    |> t.x_osz |> t.rotate Q |> t.A 0.01 |> t.rotate R
    |> raw.weierstrass
    |> (+ (10 / dim x) * t.pen x)
    |> (+ fopt)

entry schaffers_f7 (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.asy 0.5 |> t.rotate Q |> t.A 10
    |> raw.schaffers_f7
    |> (+ 10 * t.pen x)
    |> (+ fopt)

entry schaffers_f7_ill (x: []f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): f64 =
    x
    |> t.shift xopt |> t.rotate R |> t.asy 0.5 |> t.rotate Q |> t.A 1000
    |> raw.schaffers_f7
    |> (+ 10 * t.pen x)
    |> (+ fopt)

entry griewank_rosenbrock (x: []f64) (fopt: f64) (R: [][]f64): f64 =
    x
    |> t.rotate R
    |> map (* (f64.max 1 ((f64.sqrt (dim x)) / 8)))
    |> map (+ 0.5)
    |> raw.griewank_rosenbrock
    |> (+ fopt)

local def schwefel_z (x: *[]f64) (xopt: []f64): *[]f64 =
    let n = length x in
    loop acc = x for i in 1 ..< n do
        acc with [i] = acc[i] + 0.25 * (acc[i-1] - xopt[i-1])

entry schwefel (x: []f64) (xopt_sign: []f64) (fopt: f64): f64 =
    let xopt = map (* 4.2096874633/2) xopt_sign in
    let x' = map (2 *) (map2 (*) xopt_sign x) in
    let xopt2 = xopt |> map f64.abs |> map (*2) in
    let z' = schwefel_z x' xopt2 in
    let z = z' |> map2 subbed xopt2 |> t.A 10 |> map2 (+) xopt2 |> map (* 100) in
    z
    |> raw.schwefel
    |> (+ 100 * t.pen (map (/ 100) z))
    |> (+ fopt)
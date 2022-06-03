-- Implementation of BBOB functions using Futhark

-- utilities

def pow10 (i: i64) (D: i64): f64 =
    let i = f64.i64 i in
    let D = f64.i64 D in
    let factor = (i - 1) / (D - 1) in
    10 ** (6 * factor)

def sign (x: f64): f64 =
    if x == 0 then 0
    else if x > 0 then 1
    else -1

-- transformations

def Tosz_elem (x: f64): f64 =
    let x' = if x != 0 then f64.log(f64.abs(x)) else 0 in
    let c1 = if x > 0 then 10 else 5.5 in
    let c2 = if x > 0 then 7.9 else 3.1 in
    sign(x) * f64.exp (x' + 0.049 * (f64.sin (c1 * x') + f64.sin(c2 * x')))

def Tosz (x: []f64): []f64 =
    map Tosz_elem x

-- f01: sphere

entry sphere_raw (x: []f64): f64 =
    f64.sum (map2 (*) x x)

entry sphere (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    map2 (-) x xopt |> sphere_raw |> (+fopt)
    -- sphere_raw (map2 (-) x xopt) + fopt

-- f02: ellipsoidal

def ellipsoidal_factor (x: []f64) (D: i64) (i: i64): f64 =
    (pow10 (i + 1) D) * (x[i] ** 2)

entry ellipsoidal_raw (x: []f64): f64 =
    reduce (+) 0 (map (ellipsoidal_factor x (length x)) (indices x))

entry ellipsoidal (x: []f64) (xopt: []f64) (fopt: f64): f64 =
    (ellipsoidal_raw (Tosz (map2 (-) x xopt))) + fopt

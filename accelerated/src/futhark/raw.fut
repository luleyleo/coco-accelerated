import "math"

-- f01: sphere

def sphere_raw (x: []f64): f64 =
    f64.sum (map2 (*) x x)

-- f02: ellipsoidal

def ellipsoidal_factor (x: []f64) (i: i64): f64 =
    (10 ** (6 * (idr i (length x)))) * (x[i] ** 2)

def ellipsoidal_raw (x: []f64): f64 =
    f64.sum (map (ellipsoidal_factor x) (indices x))

-- f03: rastrigin

def rastrigin_raw (x: []f64): f64 =
    let D = f64.i64 (length x) in
    10 * (D - f64.sum (map f64.cos (map (2 * f64.pi *) x))) + sphere_raw(x)

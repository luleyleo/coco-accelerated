import "math"

-- f01: sphere

def sphere_raw (x: []f64): f64 =
    f64.sum (map2 (*) x x)

-- f02: ellipsoidal

def ellipsoidal_factor (x: []f64) (D: i64) (i: i64): f64 =
    (pow10 (i + 1) D) * (x[i] ** 2)

def ellipsoidal_raw (x: []f64): f64 =
    reduce (+) 0 (map (ellipsoidal_factor x (length x)) (indices x))
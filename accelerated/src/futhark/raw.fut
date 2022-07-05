import "math"

-- f01: sphere

def sphere (x: []f64): f64 =
    f64.sum (map2 (*) x x)

-- f02: ellipsoidal

local def ellipsoidal_factor (x: []f64) (i: i64): f64 =
    (10 ** (6 * (idr i (length x)))) * (x[i] ** 2)

def ellipsoidal (x: []f64): f64 =
    f64.sum (map (ellipsoidal_factor x) (indices x))

-- f03: rastrigin

def rastrigin (x: []f64): f64 =
    10 * ((dim x) - f64.sum (x |> map (2 * f64.pi *) |> map f64.cos)) + sphere(x)

-- f07: step_ellipsoidal

local def step_ellipsoidal_factor (x: []f64) (i: i64): f64 =
    (10 ** (2 * (idr i (length x)))) * (x[i] ** 2)

def step_ellipsoidal (x: []f64): f64 =
    f64.sum (map (step_ellipsoidal_factor x) (indices x))

-- f08: rosenbrock

local def rosenbrock_factor (x1: f64) (x2: f64): f64 =
    100 * (x1**2 - x2)**2 + (x1 - 1)**2

def rosenbrock (x: []f64): f64 =
    pair_indices x
    |> map (\i -> rosenbrock_factor x[i] x[i+1])
    |> f64.sum

-- f11: discus

def discus (x: []f64): f64 =
    (10**6) * (x[0]**2) + sphere x[1:]

-- f11: bent_cigar

def bent_cigar (x: []f64): f64 =
    (x[0]**2) + (10**6) * (sphere x[1:])

-- f13: sharp_ridge

def sharp_ridge (x: []f64): f64 =
    (x[0]**2) + 100 * f64.sqrt (sphere x[1:])

-- f14: different_powers

local def different_powers_factor (x: []f64) (i: i64): f64 =
    (f64.abs x[i]) ** (2 + 4 * (idr i (length x)))

def different_powers (x: []f64): f64 =
    f64.sqrt (f64.sum (map (different_powers_factor x) (indices x)))

-- f16: weierstrass

def weierstrass_factor (xi: f64) (k: f64): f64 =
    (0.5**k) * f64.cos (2 * f64.pi * (3**k) * (xi + 0.5))

def weierstrass_sum (xi: f64): f64 =
    let k = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] in
    f64.sum (map (weierstrass_factor xi) k)

def weierstrass_f0: f64 =
    weierstrass_sum 0

def weierstrass (x: []f64): f64 =
    10 * ((f64.sum (map weierstrass_sum x) - weierstrass_f0) / dim x)**3

-- f17: schaffers_f7

def schaffers_f7 (x: []f64) =
    let s = pair_indices x |> map (\i -> f64.sqrt (x[i]**2 + x[i+1]**2)) in
    let factors = map (\si -> f64.sqrt(si) * (1 + f64.sin(50 * si**0.2)**2)) s in
    ((f64.sum factors) / (dim x - 1))**2

-- f19: griewank_rosenbrock

def griewank_rosenbrock (x: []f64): f64 =
    pair_indices x
    |> map (\i -> (rosenbrock_factor x[i] x[i+1]))
    |> map (\xi -> (xi) / 4000 - f64.cos(xi))
    |> f64.sum
    |> (* 10 / (dim x - 1))
    |> (+ 10)

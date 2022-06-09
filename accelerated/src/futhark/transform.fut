import "math"

def x_osz_elem (x: f64): f64 =
    let x' = if x != 0 then f64.log(f64.abs(x)) else 0 in
    let c1 = if x > 0 then 10 else 5.5 in
    let c2 = if x > 0 then 7.9 else 3.1 in
    sign(x) * f64.exp (x' + 0.049 * (f64.sin (c1 * x') + f64.sin(c2 * x')))

def x_osz (x: []f64): []f64 =
    map x_osz_elem x

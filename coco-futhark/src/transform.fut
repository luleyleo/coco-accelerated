import "math"

def shift (xopt: []f64) (x: []f64): []f64 =
    map2 (-) x xopt


local def x_osz_elem (x: f64): f64 =
    let x' = if x != 0 then f64.log(f64.abs(x)) else 0 in
    let c1 = if x > 0 then 10 else 5.5 in
    let c2 = if x > 0 then 7.9 else 3.1 in
    sign(x) * f64.exp (x' + 0.049 * (f64.sin (c1 * x') + f64.sin(c2 * x')))

def x_osz (x: []f64): []f64 =
    map x_osz_elem x

def y_osz (y: f64): f64 = x_osz_elem y


local def asy_elem (b: f64) (x: []f64) (i: i64): f64 =
    let xi = x[i] in
    if xi > 0
    then xi ** (1 + b * (idr i (length x)) * (f64.sqrt xi))
    else xi

def asy (b: f64) (x: []f64): []f64 =
    map (asy_elem b x) (indices x)


local def A_elem (a: f64) (x: []f64) (i: i64): f64 =
    f64.sqrt(a) ** (idr i (length x))

def A (a: f64) (x: []f64): []f64 =
    let factors = map (A_elem a x) (indices x) in
    map2 (*) factors x


def pen (x: []f64): f64 =
    x
    |> map f64.abs
    |> map (subbed 5)
    |> map (f64.max 0)
    |> map (**2)
    |> f64.sum


def rotate (R: [][]f64) (x: []f64): []f64 =
    mat'vec R x

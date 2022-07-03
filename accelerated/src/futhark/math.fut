
def dim (x: []f64): f64 =
    f64.i64 (length x)

-- like (-) but the parameters are swapped
def subbed (a: f64) (b: f64) = b - a

-- the index dimension ratio, or whatever you want to call this
def idr (i: i64) (d: i64): f64 =
    let I = f64.i64 i in
    let D = f64.i64 d in
    I / (D - 1)

def sign (x: f64): f64 =
    if x == 0 then 0
    else if x > 0 then 1
    else -1

def sign' (x: f64): f64 =
    if x > 0 then 1
    else -1

def mat'mat [n][m][p] (A: [n][m]f64) (B: [m][p]f64) : [n][p]f64 =
    map (\A_row ->
            map (\B_col ->
                    reduce (+) 0 (map2 (*) A_row B_col))
                (transpose B))
        A

def mat'vec [n] (M: [n][n]f64) (V: [n]f64) : [n]f64 =
    M |> map (map2 (*) V) |> map f64.sum
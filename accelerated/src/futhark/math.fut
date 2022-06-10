
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

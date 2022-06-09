
def pow10 (i: i64) (D: i64): f64 =
    let i = f64.i64 i in
    let D = f64.i64 D in
    let factor = (i - 1) / (D - 1) in
    10 ** (6 * factor)

def sign (x: f64): f64 =
    if x == 0 then 0
    else if x > 0 then 1
    else -1
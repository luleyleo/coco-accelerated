module bbob = import "bbob"

entry sphere (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.sphere x xopt fopt)

entry ellipsoidal (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.ellipsoidal x xopt fopt)

entry rastrigin (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.rastrigin x xopt fopt)

entry bueche_rastrigin (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.bueche_rastrigin x xopt fopt)

entry linear_slope (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.linear_slope x xopt fopt)

entry attractive_sector (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.attractive_sector x xopt fopt R Q)

entry step_ellipsoidal (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.step_ellipsoidal x xopt fopt R Q)

entry rosenbrock (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.rosenbrock x xopt fopt)

entry rosenbrock_rotated (x: [][]f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.rosenbrock_rotated x fopt R)

entry ellipsoidal_rotated (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.ellipsoidal_rotated x xopt fopt R)

entry discus (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.discus x xopt fopt R)

entry bent_cigar (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.bent_cigar x xopt fopt R)

entry sharp_ridge (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.sharp_ridge x xopt fopt R Q)

entry different_powers (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.different_powers x xopt fopt R)

entry rastrigin_rotated (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.rastrigin_rotated x xopt fopt R Q)

entry weierstrass (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.weierstrass x xopt fopt R Q)

entry schaffers_f7 (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.schaffers_f7 x xopt fopt R Q)

entry schaffers_f7_ill (x: [][]f64) (xopt: []f64) (fopt: f64) (R: [][]f64) (Q: [][]f64): []f64 =
    x |> map (\x -> bbob.schaffers_f7_ill x xopt fopt R Q)

entry griewank_rosenbrock (x: [][]f64) (fopt: f64) (R: [][]f64): []f64 =
    x |> map (\x -> bbob.griewank_rosenbrock x fopt R)

entry schwefel (x: [][]f64) (xopt: []f64) (fopt: f64): []f64 =
    x |> map (\x -> bbob.schwefel x xopt fopt)

import numpy

from mcdmrs import multicriterion_flow


def make_prom_inputs(
    n: int, m: int, seed: int = 42, max_value=10.0
) -> tuple[numpy.ndarray, numpy.ndarray, numpy.ndarray]:
    numpy.random.seed(seed)  # make our randomness reproducible
    matrix = numpy.random.uniform(-max_value, max_value, (int(n), int(m)))
    _weights = numpy.random.uniform(1, 5, int(m))
    weights = _weights / _weights.sum()
    types = numpy.random.choice([-1, 1], int(m))
    return matrix, weights, types


def test_it():
    mat_t = numpy.array([[0.8, 0.2, 0.5], [0.8, 0.2, 0.5]]).T  # // array
    types = numpy.array([1, 1])
    prefs = numpy.array(["usual", "usual"])
    q = numpy.array([0.0, 0.0])
    p = numpy.array([0.0, 0.0])

    plus, minus = multicriterion_flow(
        mat_t.astype("float32"),
        types,
        prefs,
        q.astype("float32"),
        p.astype("float32"),
    )
    exp_plus = numpy.array([[1.0, 0.0, 0.5], [1.0, 0.0, 0.5]]).T
    exp_minus = numpy.array([[0.0, 1.0, 0.5], [0.0, 1.0, 0.5]]).T

    assert numpy.abs(plus - exp_plus).sum() <= 1e-6, (plus, exp_plus)
    assert numpy.abs(minus - exp_minus).sum() <= 1e-6, (minus, exp_minus)


def test_multicriterion_flow():
    mat = numpy.array(
        [
            [-2.51, 9.01, 4.64, 1.97, -6.88, -6.88, -8.84, 7.32],
            [2.02, 4.16, -9.59, 9.4, 6.65, -5.75, -6.36, -6.33],
            [-3.92, 0.5, -1.36, -4.18, 2.24, -7.21, -4.16, -2.67],
            [-0.88, 5.7, -6.01, 0.28, 1.85, -9.07, 2.15, -6.59],
            [-8.7, 8.98, 9.31, 6.17, -3.91, -8.05, 3.68, -1.2],
            [-7.56, -0.1, -9.31, 8.19, -4.82, 3.25, -3.77, 0.4],
            [0.93, -6.3, 9.39, 5.5, 8.79, 7.9, 1.96, 8.44],
            [-8.23, -6.08, -9.1, -3.49, -2.23, -4.57, 6.57, -2.86],
            [-4.38, 0.85, -7.18, 6.04, -8.51, 9.74, 5.44, -6.03],
            [-9.89, 6.31, 4.14, 4.58, 5.43, -8.52, -2.83, -7.68],
            [7.26, 2.47, -3.38, -8.73, -3.78, -3.5, 4.59, 2.75],
            [7.74, -0.56, -7.61, 4.26, 5.22, 1.23, 5.42, -0.12],
            [0.45, -1.45, -9.49, -7.84, -9.37, 2.73, -3.71, 0.17],
            [8.15, -5.01, -1.79, 5.11, -5.42, -8.46, -4.2, -6.78],
            [8.59, 6.16, 2.67, 7.43, 6.07, -6.27, 7.85, 0.79],
            [6.15, 7.92, -3.64, -7.8, -5.44, -1.46, 6.36, 7.21],
            [-9.86, 0.21, -1.65, -5.56, -7.6, -3.25, 8.86, -3.54],
            [0.38, 4.06, -2.73, 9.44, 9.25, -4.96, -0.06, -3.98],
            [-4.3, -9.26, 2.19, 0.05, -8.97, -4.43, 8.17, -5.21],
            [-7.1, -0.21, 9.71, -5.16, 3.44, 5.23, -5.25, 4.56],
        ]
    )

    weight = numpy.array([0.11, 0.157, 0.158, 0.14, 0.061, 0.194, 0.102, 0.078])

    criteria_type = numpy.array([-1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0])
    pref_function = numpy.array(
        [
            "vshape2",
            "usual",
            "ushape",
            "vshape",
            "usual",
            "level",
            "vshape2",
            "usual",
        ]
    )

    q = numpy.array([0.37, 0.95, 0.73, 0.6, 0.16, 0.16, 0.06, 0.87])
    p = numpy.array([0.6, 0.71, 0.02, 0.97, 0.83, 0.21, 0.18, 0.18])

    plus_t, minus_t = multicriterion_flow(
        mat,
        criteria_type,
        pref_function,
        q,
        p,
    )

    score = (plus_t * weight).sum(axis=1) - (minus_t * weight).sum(axis=1)

    exp_promii = numpy.array(
        [
            0.05642106,
            -0.17198806,
            -0.07260072,
            -0.4366739,  # // <- min
            0.02175211,
            0.29782811,
            0.50030197,  # // <- max
            -0.06959151,
            0.1629708,
            -0.13237331,
            -0.21495115,
            -0.05180758,
            0.0496468,
            -0.07426273,
            -0.18974575,
            -0.23589474,
            0.03275095,
            -0.05369158,
            0.14787299,
            0.43403623,
        ]
    )

    assert numpy.abs(score - exp_promii).sum() < 1e-5, score


def test_mc_flow_timing():
    mat, weight, types = make_prom_inputs(10_000, 8)

    plus_t, minus_t = multicriterion_flow(
        mat,
        types,
        ["usual"] * mat.shape[1],
        numpy.zeros(mat.shape[1], dtype="float32"),
        numpy.zeros(mat.shape[1], dtype="float32"),
    )

    score = (plus_t * weight).sum(axis=1) - (minus_t * weight).sum(axis=1)

    assert (
        numpy.abs(
            score[:5] - [-0.17915933, -0.20779199, 0.19458257, 0.06256933, 0.19036096]
        ).sum()
        <= 1e-4
    )

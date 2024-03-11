import numpy

from .._mcdmrs import _multicriterion_flow

from typing import TypeVar

DType = TypeVar("DType", bound=numpy.dtype)
T1 = TypeVar("T1", bound=int)
T2 = TypeVar("T2", bound=int)


def multicriterion_flow(
    matrix: numpy.ndarray[tuple[T1, T2], DType],
    criteria_type: numpy.ndarray[tuple[T2], DType] | list[int] | None = None,
    pref_function: str | numpy.ndarray[tuple[T2], DType] | list[str] | None = None,
    q: numpy.ndarray[tuple[T2], DType] | list[int | float] | None = None,
    p: numpy.ndarray[tuple[T2], DType] | list[int | float] | None = None,
):
    plus_t, minus_t = _multicriterion_flow(
        (matrix * criteria_type).astype("float32").T,
        pref_function,
        numpy.array(q, dtype="float32"),
        numpy.array(p, dtype="float32"),
    )

    return plus_t.T, minus_t.T

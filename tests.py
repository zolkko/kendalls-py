from kendallspy import taub as kendalltau_rs
from scipy.stats import kendalltau
import numpy as np
import pytest


sizes = np.random.randint(2, 100, 10_000)
testdata = [(np.random.rand(size), np.random.rand(size)) for size in sizes]


@pytest.mark.parametrize("a,b", testdata)
def test_compare_taubs(a, b):
    res1 = kendalltau(a, b)
    res2 = kendalltau_rs(a, b)
    assert res1.correlation == pytest.approx(res2)

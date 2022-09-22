from pathlib import Path
from typing import Any, Dict, List
import cbor2
import pandas as pnd


class ConfidenceInterval:
    level: float
    lower_bound: float
    upper_bound: float

    def __init__(self, obj: Dict[str, float]) -> None:
        self.level = obj['confidence_level']
        self.lower_bound = obj['lower_bound']
        self.upper_bound = obj['upper_bound']


class Estimate:
    interval: ConfidenceInterval
    point: float
    std_error: float

    def __init__(self, obj: Dict[str, Any]) -> None:
        self.interval = ConfidenceInterval(obj['confidence_interval'])
        self.point = obj['point_estimate']
        self.std_error = obj['standard_error']


class Measurements:
    datetime: str
    iterations: List[float]
    values: List[float]
    avg_values: List[float]
    throughput: int
    estimates: Dict[str, Estimate]

    def __init__(self, obj: Dict[str, Any]) -> None:
        self.datetime = str
        self.iterations = obj['iterations']
        self.values = obj['values']
        self.avg_values = obj['avg_values']

        self.estimates = dict()
        for name, estimate in obj['estimates'].items():
            if estimate != None:
                self.estimates[name] = Estimate(estimate)

        self.throughput = obj['throughput']['Elements']


class Benchmark:
    function: str
    target: str
    dimension: int
    batch_size: int

    measurements: Measurements

    def __init__(self, path: Path) -> None:
        with open(path / 'benchmark.cbor', 'rb') as file:
            obj = cbor2.decoder.load(file)

            self.function = obj['id']['group_id']
            self.target = obj['id']['function_id']

            dimension, batch_size = obj['id']['value_str'].split('x')
            self.dimension = int(dimension)
            self.batch_size = int(batch_size)

            latest = obj['latest_record']

        with open(path / latest, 'rb') as file:
            obj = cbor2.decoder.load(file)
            self.measurements = Measurements(obj)


def load_report(path: Path) -> List[Benchmark]:
    base = path / 'data' / 'main'
    paths = base.glob('**/benchmark.cbor')
    benchmarks = [Benchmark(path.parent) for path in paths]

    return benchmarks


def make_report_data_frame(benchmarks: List[Benchmark]) -> pnd.DataFrame:
    targets = [b.target for b in benchmarks]
    functions = [b.function for b in benchmarks]
    dimensions = [b.dimension for b in benchmarks]
    batch_sizes = [b.batch_size for b in benchmarks]
    medians = [b.measurements.estimates['median'].point for b in benchmarks]
    means = [b.measurements.estimates['mean'].point for b in benchmarks]

    return pnd.DataFrame({
        'target': targets,
        'function': functions,
        'dimension': dimensions,
        'batch_size': batch_sizes,
        'median': medians,
        'mean': means,
    })


def load_report_data_frame(path: Path) -> pnd.DataFrame:
    return make_report_data_frame(load_report(path))

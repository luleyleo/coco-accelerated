from collections import defaultdict
from pathlib import Path
from typing import Any, DefaultDict, Dict, List
from datetime import datetime
import cbor2


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
        self.point = obj['point']
        self.std_error = obj['std_error']


class Measurements:
    datetime: datetime
    iterations: List[float]
    values: List[float]
    avg_values: List[float]
    throughput: int
    estimates: Dict[str, Estimate]

    def __init__(self, obj: Dict[str, Any]) -> None:
        self.datetime = datetime.fromisoformat(obj['datetime'])
        self.iterations = obj['iterations']
        self.values = obj['values']
        self.avg_values = obj['avg_values']

        self.estimates = dict()
        for name, estimate in obj['estimates'].items():
            self.estimates[name] = Estimate(estimate)

        self.throughput = obj['throughput']['Elements']


class Benchmark:
    group_id: str
    function_id: str
    dimension: int

    measurements: Measurements

    def __init__(self, path: Path) -> None:
        with open(path / 'benchmark.cbor', 'rb') as file:
            obj = cbor2.decoder.load(file)

            self.group_id = obj['group_id']
            self.function_id = obj['function_id']
            self.dimension = int(obj['value_str'])

            latest = obj['latest_record']

        with open(path/ latest, 'rb') as file:
            obj = cbor2.decoder.load(file)
            self.measurements = Measurements(obj)



class Target:
    dimensions: DefaultDict[int, Benchmark]

    def __init__(self) -> None:
        self.dimensions = defaultdict()


class Function:
    targets: DefaultDict[int, Target]

    def __init__(self) -> None:
        self.targets = defaultdict()


class Report:
    functions: DefaultDict[str, Function]

    def __init__(self) -> None:
        self.functions = defaultdict()


def load_report(path: Path) -> Report:
    base = path / 'data' / 'main'
    paths = base.glob('**/benchmark.cbor')
    paths = (path.parent for path in paths)
    benchmarks = [Benchmark(path) for path in paths]

    report = Report()
    for benchmark in benchmarks:
        report[benchmark.group_id][benchmark.function_id]


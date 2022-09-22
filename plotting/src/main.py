from pathlib import Path
import sys
from report import load_report_data_frame
from augment import augment_report_data_frame

import plots


OUT = Path('plots')


def main():
    args = sys.argv
    all = len(args) == 0

    report = load_report_data_frame(Path('../reports/current-09-09'))
    report = augment_report_data_frame(report)

    OUT.mkdir(parents=True, exist_ok=True)

    if all or 'batch_scaling' in args:
        plots.batch_scaling(report).savefig(OUT / 'batch_scaling.svg')

    if all or 'delta' in args:
        plots.delta(report).savefig(OUT / 'delta.svg')

    if all or 'aggregation' in args:
        plots.aggregation(report).savefig(OUT / 'aggregation.svg')


if __name__ == '__main__':
    main()

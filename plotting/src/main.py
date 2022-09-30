from pathlib import Path
import sys
from report import load_report_data_frame
from augment import augment_report_data_frame

import plots


OUT = Path('plots')


def save(figure, name):
    figure.savefig(OUT / f'{name}.png', format='png', bbox_inches='tight')


def main():
    args = sys.argv
    all = len(args) == 1

    report = load_report_data_frame(Path('../reports/current-09-09'))
    report = augment_report_data_frame(report)

    OUT.mkdir(parents=True, exist_ok=True)

    if all or 'batch_scaling' in args:
        save(plots.batch_scaling(report), 'batch_scaling')

    if all or 'delta' in args:
        save(plots.delta(report), 'delta')

    if all or 'aggregation' in args:
        save(plots.aggregation(report), 'aggregation')

    if all or 'deviation_by_batch_size' in args:
        save(plots.deviation_by_batch_size(report), 'deviation_by_batch_size')

    if all or 'deviation_by_function' in args:
        save(plots.deviation_by_function(report), 'deviation_by_function')


if __name__ == '__main__':
    main()

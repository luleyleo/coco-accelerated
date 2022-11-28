from pathlib import Path
import sys
from report import load_report_data_frame
from augment import augment_report_data_frame

import plots


def delta_threading_cuda_1080(report):
    return plots.delta_threading(report, target='cuda-1080')

def delta_threading_cuda_v100s(report):
    return plots.delta_threading(report, target='cuda-v100s')


REPORT = 'current-both'
OUT = Path('.') / '..' / 'plots'
PLOTS = [
    plots.delta_reference,
    plots.delta_reference_40x128,
    plots.delta_reference_40x8,
    plots.delta_reference_2x128,
    plots.delta_threading,
    delta_threading_cuda_1080,
    delta_threading_cuda_v100s,
    plots.aggregation,
    plots.aggregation_2,
    plots.aggregation_40,
    plots.deviation_by_batch_size,
    plots.deviation_by_function,
    plots.real_world_overview,
]


def save(figure, name):
    figure.savefig(OUT / f'{name}.pdf', format='pdf', bbox_inches='tight')


def main():
    args = sys.argv
    all = len(args) == 1

    report = load_report_data_frame(Path('../reports/' + REPORT))
    report = augment_report_data_frame(report)

    OUT.mkdir(parents=True, exist_ok=True)

    for plot in PLOTS:
        plot_name = plot.__name__
        if all or plot_name in args:
            print('plotting ' + plot_name)
            save(plot(report), plot_name)


if __name__ == '__main__':
    main()

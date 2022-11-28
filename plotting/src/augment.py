import pandas as pnd


reference_index = sorted(['cuda-v100s', 'cuda-1080', 'multicore', 'reference', 'sequential']).index('reference')


def calculate_normalized_median(report: pnd.DataFrame) -> pnd.Series:
    def use_second(g: pnd.Series):
        new = pnd.Series(5 * [g.iloc[reference_index]])
        new.index = g.index
        return new

    return report['median'] / report.groupby(['function', 'dimension', 'batch_size'])['median'].apply(use_second)


def calculate_normalized_delta(report: pnd.DataFrame) -> pnd.Series:
    s1 = report[report['target'] == 'sequential']['median_n']
    s2 = report[report['target'] == 'multicore']['median_n']
    s3 = report[report['target'] == 'cuda-v100s']['median_n']
    s4 = report[report['target'] == 'cuda-1080']['median_n']

    r1 = report[report['target'] == 'reference']['median_n']
    r2 = report[report['target'] == 'reference']['median_n']
    r3 = report[report['target'] == 'reference']['median_n']
    r4 = report[report['target'] == 'reference']['median_n']

    r1.index = s1.index
    r2.index = s2.index
    r3.index = s3.index
    r4.index = s4.index

    d1 = s1 - r1
    d2 = s2 - r2
    d3 = s3 - r3
    d4 = s4 - r4

    return d1.add(d2, fill_value=0).add(d3, fill_value=0).add(d4, fill_value=0)


def calculate_normalized_deviation(report: pnd.DataFrame) -> pnd.Series:
    return report['std_dev'] / report['median']


def augment_report_data_frame(report: pnd.DataFrame) -> pnd.DataFrame:
    report['median_n'] = calculate_normalized_median(report)
    report['delta'] = calculate_normalized_delta(report)
    report['std_dev_n'] = calculate_normalized_deviation(report)

    return report


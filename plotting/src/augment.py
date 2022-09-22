import pandas as pnd


def calculate_normalized_median(report: pnd.DataFrame) -> pnd.Series:
    return report['median'] / report.groupby(['function', 'dimension'])['median'].transform('max')


def calculate_normalized_delta(report: pnd.DataFrame) -> pnd.Series:
    s1 = report[report['target'] == 'sequential']['median_n']
    s2 = report[report['target'] == 'multicore']['median_n']

    r1 = report[report['target'] == 'reference']['median_n']
    r2 = report[report['target'] == 'reference']['median_n']

    r1.index = s1.index
    r2.index = s2.index

    d1 = s1 - r1
    d2 = s2 - r2

    return d1.add(d2, fill_value=0)


def augment_report_data_frame(report: pnd.DataFrame) -> pnd.DataFrame:
    report['median_n'] = calculate_normalized_median(report)
    report['delta'] = calculate_normalized_delta(report)

    return report


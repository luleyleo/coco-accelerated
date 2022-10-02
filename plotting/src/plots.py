from matplotlib.figure import Figure

import seaborn as sbn
import matplotlib.pyplot as plt
import pandas as pnd


def delta(report: pnd.DataFrame, dimension = 40, batch_size = 128) -> Figure:
    width = 1

    data = report[(report['dimension'] == dimension) & (report['batch_size'] == batch_size)]
    data = data.sort_values(by='delta')
    data = data[(data['target'] == 'sequential') | (data['target'] == 'multicore')]

    # Initialize the matplotlib figure
    f, ax = plt.subplots(figsize=(8, 10))

    sbn.barplot(x="delta", y="function", hue='target', data=data, width=0.5, errorbar=None)

    # Add a legend and informative axis label
    ax.legend(ncol=1, loc="lower right", frameon=True)
    ax.set(xlim=(-width, width), ylabel="", xlabel="Relative Improvement of D40xB128")
    ax.figure.subplots_adjust(left = 0.2)
    sbn.despine(left=True, bottom=True)

    return f


def batch_scaling(report: pnd.DataFrame) -> Figure:
    return sbn.relplot(data=report, kind='line', x='batch_size', y='median_n', hue='target', row='function', col='dimension')


def aggregation(report: pnd.DataFrame, dimension = 40) -> Figure:
    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'median': 'mean'}).reset_index()

    return sbn.barplot(data[data['dimension'] == dimension], x='batch_size', y='median', hue='target')


def deviation_by_batch_size(report: pnd.DataFrame, dimension = 40) -> Figure:
    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'std_dev': 'mean'}).reset_index()

    return sbn.barplot(data[data['dimension'] == dimension], x='batch_size', y='std_dev', hue='target')


def deviation_by_function(report: pnd.DataFrame, dimension = 40) -> Figure:
    data = report.groupby(by=['dimension', 'target', 'function']).aggregate({'std_dev': 'mean'}).reset_index()

    sbn.set(rc={"figure.figsize":(6, 8)})
    fig = sbn.barplot(data[data['dimension'] == dimension], x='std_dev', y='function', hue='target')
    sbn.set()

    return fig


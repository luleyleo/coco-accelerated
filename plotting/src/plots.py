from pathlib import Path
from matplotlib.figure import Figure

import seaborn as sbn
import matplotlib.pyplot as plt
import pandas as pnd


def delta(report: pnd.DataFrame, dimension = 40, batch_size = 64) -> Figure:
    width = 1

    data = report[(report['dimension'] == dimension) & (report['batch_size'] == batch_size)]
    data = data.sort_values(by='delta')
    data_s = data[(data['target'] == 'sequential')]
    data_m = data[(data['target'] == 'multicore')]

    # Initialize the matplotlib figure
    f, ax = plt.subplots(figsize=(8, 10))

    # Plot the total crashes
    sbn.set_color_codes("pastel")
    sbn.barplot(x="delta", y="function", data=data_s, label="Sequential", errorbar=None, color="b")

    # Plot the crashes where alcohol was involved
    sbn.set_color_codes("muted")
    sbn.barplot(x="delta", y="function", data=data_m, label="Multicore", errorbar=None, color="b")

    # Add a legend and informative axis label
    ax.legend(ncol=1, loc="lower right", frameon=True)
    ax.set(xlim=(-width, width), ylabel="", xlabel="Relative Improvement of D40xB128")
    sbn.despine(left=True, bottom=True)

    return f


def batch_scaling(report: pnd.DataFrame) -> Figure:
    return sbn.relplot(data=report, kind='line', x='batch_size', y='median_n', hue='target', row='function', col='dimension')


def aggregation(report: pnd.DataFrame) -> Figure:
    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'median': 'sum'}).reset_index()

    fig, ax = plt.subplots(1, 2, figsize=(12, 7))

    ax[0].set_title('dimension 2')
    sbn.barplot(data[data['dimension'] == 2], x='batch_size', y='median', hue='target', ax=ax[0])

    ax[1].set_title('dimension 40')
    sbn.barplot(data[data['dimension'] == 40], x='batch_size', y='median', hue='target', ax=ax[1])

    return fig


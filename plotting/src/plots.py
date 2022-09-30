from pathlib import Path
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


def aggregation(report: pnd.DataFrame) -> Figure:
    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'median': 'mean'}).reset_index()

    fig, ax = plt.subplots(1, 2, figsize=(12, 7))

    ax[0].set_title('dimension 2')
    sbn.barplot(data[data['dimension'] == 2], x='batch_size', y='median', hue='target', ax=ax[0])

    ax[1].set_title('dimension 40')
    sbn.barplot(data[data['dimension'] == 40], x='batch_size', y='median', hue='target', ax=ax[1])

    return fig


def deviation_by_batch_size(report: pnd.DataFrame) -> Figure:
    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'std_dev': 'mean'}).reset_index()

    fig, ax = plt.subplots(1, 2, figsize=(12, 7))

    ax[0].set_title('dimension 2')
    sbn.barplot(data[data['dimension'] == 2], x='batch_size', y='std_dev', hue='target', ax=ax[0])

    ax[1].set_title('dimension 40')
    sbn.barplot(data[data['dimension'] == 40], x='batch_size', y='std_dev', hue='target', ax=ax[1])

    return fig


def deviation_by_function(report: pnd.DataFrame) -> Figure:
    data = report.groupby(by=['dimension', 'target', 'function']).aggregate({'std_dev': 'mean'}).reset_index()

    fig, ax = plt.subplots(1, 2, figsize=(26, 12))

    ax[0].set_title('dimension 2')
    sbn.barplot(data[data['dimension'] == 2], x='std_dev', y='function', hue='target', ax=ax[0])

    ax[1].set_title('dimension 40')
    sbn.barplot(data[data['dimension'] == 40], x='std_dev', y='function', hue='target', ax=ax[1])

    return fig


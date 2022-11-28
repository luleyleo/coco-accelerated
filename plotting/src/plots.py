from typing import List
from matplotlib.figure import Figure

import seaborn as sbn
import matplotlib.pyplot as plt
import pandas as pnd


def new_figure(width=6, height=8):
    return plt.subplots(figsize = (width, height))


def delta_reference(report: pnd.DataFrame, dimension = None, batch_size = None) -> Figure:
    f, ax = new_figure(width = 8, height = 10)

    data = report

    if dimension != None:
        data = data[(data['dimension'] == dimension)]

    if batch_size != None:
        data = data[(data['batch_size'] == batch_size)]

    data = data.sort_values(by='delta')
    data = data[data['target'] != 'reference']


    sbn.barplot(x="delta", y="function", hue='target', data=data, width=0.5, errorbar=None)

    width = 1
    ax.legend(ncol=1, loc="lower right", frameon=True)
    ax.set(xlim=(-width, width), ylabel="", xlabel="")
    ax.figure.subplots_adjust(left = 0.2)
    sbn.despine(left=True, bottom=True)

    return f

def delta_reference_40x128(report: pnd.DataFrame) -> Figure:
    return delta_reference(report, 40, 128)

def delta_reference_40x8(report: pnd.DataFrame) -> Figure:
    return delta_reference(report, 40, 8)

def delta_reference_2x128(report: pnd.DataFrame) -> Figure:
    return delta_reference(report, 2, 128)


def delta_threading(report: pnd.DataFrame, dimension = 40, batch_size = 128, target = 'multicore') -> Figure:
    f, ax = new_figure(width = 8, height = 10)

    data = report
    data = data[(data['dimension'] == dimension) & (data['batch_size'] == batch_size)]
    data = data[(data['target'] == 'sequential') | (data['target'] == target)]
    data = data.sort_values(by=['function', 'target'])

    single = data[data['target'] == 'sequential'].reset_index()
    multi = data[data['target'] == target].reset_index()

    single['delta'] = multi['median_n'] - single['median_n']

    data = single[['function', 'delta']]
    data = data.sort_values(by='delta', ascending=True)

    colors = list()
    for idx, row in data.iterrows():
        if row['delta'] < -0.5:
            colors.append('high')
        elif row['delta'] < -0.05:
            colors.append('medium')
        else:
            colors.append('low')
    data['color'] = colors

    palette = {"high":"tab:green",
            "medium":"tab:orange",
            "low":"tab:red"}

    sbn.barplot(x="delta", y="function", data=data, errorbar=None, hue='color', palette=palette)

    ax.set(xlim=(-1, 0.25), ylabel="", xlabel="")
    ax.figure.subplots_adjust(left = 0.2)
    sbn.despine(left=True, bottom=True)

    return f


def batch_scaling(report: pnd.DataFrame, dimension = None) -> List[Figure]:
    functions = set(report['function'])
    data = report[report['dimension'] == dimension]
    return [sbn.relplot(data=data[data['function'] == f], kind='line', x='batch_size', y='median_n', hue='target') for f in functions]


def aggregation(report: pnd.DataFrame, dimension = None) -> Figure:
    fig, ax = new_figure(height=6)

    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'median': 'median'}).reset_index()

    if dimension != None:
        data = data[data['dimension'] == dimension]

    sbn.barplot(data, x='batch_size', y='median', hue='target')

    return fig

def aggregation_2(report: pnd.DataFrame) -> Figure:
    return aggregation(report, dimension=2)

def aggregation_40(report: pnd.DataFrame) -> Figure:
    return aggregation(report, dimension=40)


def deviation_by_batch_size(report: pnd.DataFrame, dimension = None) -> Figure:
    fig, ax = new_figure(height=6)

    data = report.groupby(by=['dimension', 'batch_size', 'target']).aggregate({'std_dev_n': 'median'}).reset_index()

    if dimension != None:
        data = data[data['dimension'] == dimension]

    sbn.barplot(data, x='batch_size', y='std_dev_n', hue='target', ax=ax)
    ax.set_ylabel('std_dev')

    return fig


def deviation_by_function(report: pnd.DataFrame, dimension = None) -> Figure:
    fig, ax = new_figure(height=16)

    data = report.groupby(by=['dimension', 'target', 'function']).aggregate({'std_dev': 'mean'}).reset_index()

    if dimension != None:
        data = data[data['dimension'] == dimension]

    sbn.barplot(data, x='std_dev', y='function', hue='target')

    return fig


def real_world_overview(report : pnd.DataFrame) -> Figure:
    fig, ax = new_figure(height=6)

    index = ['reference', 'sequential', 'multicore', 'multicore-total', 'cuda-1080']
    data = {
        '16 batch':  [ 43,  35,  27, 135,  60],
        '128 batch': [240, 155, 102, 665, 108],
    }

    dt = {
        'target': [],
        'batch': [],
        'time': [],
    }

    for batch, times in data.items():
        for i in range(len(index)):
            dt['target'].append(index[i])
            dt['batch'].append(batch)
            dt['time'].append(times[i])

    df = pnd.DataFrame(data=dt)

    sbn.barplot(df, x='batch', y='time', hue='target')

    return fig


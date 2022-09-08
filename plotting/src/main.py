import cbor2

def load_benchmark(path: str):
    with open(path, 'rb') as file:
        obj = cbor2.decoder.load(file)

    return obj

if __name__ == '__main__':
    test = load_benchmark('reports/current/data/main/Sphere-d20/futhark_c/50/measurement_220907190756.cbor')
    pass

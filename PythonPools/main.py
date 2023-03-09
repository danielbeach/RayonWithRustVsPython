from datetime import datetime
import os
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor

def read_file(file_name):
    with open(file_name) as f:
        data = f.readlines()
    return data

def convert_to_tsv(lines):
    new_lines = []
    for line in lines:
        new_line = line.replace(',', '\t')
        new_lines.append(new_line)
    return new_lines

def write_file(lines, path):
    with open(path, 'w') as f:
        f.writelines(lines)

def divide_chunks(l, n):
    for i in range(0, len(l), n):
        yield l[i:i + n]

def main():
    t1 = datetime.now()
    lines = read_file('divvy-tripdata.csv')
    lines_chunks = list(divide_chunks(lines, 2000000))
    lines_chunks = [lines_chunks[0], lines_chunks[1], lines_chunks[2]]
    with ProcessPoolExecutor(max_workers=3) as executor:
        new_lines = executor.map(convert_to_tsv, lines_chunks)
    unwrap_futre = [item for sublist in new_lines for item in sublist]
    write_file(unwrap_futre, path='divvy-tripdata.tsv')
    t2 = datetime.now()
    print(f'Finished in {t2 - t1}')
    

if __name__ == '__main__':
    main()

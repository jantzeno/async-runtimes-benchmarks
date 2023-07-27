#!/usr/bin/env python3
from datetime import timedelta
import json
import os
import sys


def main(argv):

    relativePath, _ = os.path.split(os.path.realpath(__file__))
    inputFile = os.path.join(relativePath, "results.json")

    if not os.path.isfile(inputFile):
        print("results.json file not found in " + relativePath)
        sys.exit()

    with open(inputFile, "r") as f:
        data = json.load(f)
        statData_10k = [result for result in data if result["benchmark"]
                        == "stat" and int(result["threadCount"]) == 10000]
        hashData_10k = [result for result in data if result["benchmark"]
                        == "hash" and int(result["threadCount"]) == 10000]
        statData_100k = [result for result in data if result["benchmark"]
                         == "stat" and int(result["threadCount"]) == 100000]
        hashData_100k = [result for result in data if result["benchmark"]
                         == "hash" and int(result["threadCount"]) == 100000]
        statData_1m = [result for result in data if result["benchmark"]
                       == "stat" and int(result["threadCount"]) == 1000000]
        hashData_1m = [result for result in data if result["benchmark"]
                       == "hash" and int(result["threadCount"]) == 1000000]

    statTable_10k = generateTable(statData_10k)
    statTable_100k = generateTable(statData_100k)
    statTable_1m = generateTable(statData_1m)
    hashTable_10k = generateTable(hashData_10k)
    hashTable_100k = generateTable(hashData_100k)
    hashTable_1m = generateTable(hashData_1m)

    outputFile = os.path.join(relativePath, "tables.md")
    with open(outputFile, "w") as f:
        f.write("stat_10k\n")
        f.writelines(statTable_10k)
        f.write("\nstat_100k\n")
        f.writelines(statTable_100k)
        f.write("\nstat_1m\n")
        f.writelines(statTable_1m)
        f.write("\nhash_10k\n")
        f.writelines(hashTable_10k)
        f.write("\nhash_100k\n")
        f.writelines(hashTable_100k)
        f.write("\nhash_1m\n")
        f.writelines(hashTable_1m)


def generateTable(results):
    data = []
    categories = []

    header = "| Header | Real Time (seconds) | Memory (MiB) |\n"
    separator = "| --- | :---: | :---: |\n"
    rows = []

    for result in results:
        name = ""
        if result["language"] == "rust":
            name = result["language"].capitalize() + " " + \
                result["library"] + " " + result["method"]
        elif result["language"] == "java":
            name = result["language"].capitalize() + " " + result["method"]
        else:
            name = result["language"].capitalize()

        time = float(result["realTime"])

        if result["memoryUnit"] == "GiB":
            memAmount = float(result["memory"]) * 1024
        else:
            memAmount = float(result["memory"])
        memory = str(memAmount)

        row = f"| {name} | {time:.3f} | {memory} |\n"
        rows.append(row)

    rows = sorted(rows)
    rows.insert(0, header)
    rows.insert(1, separator)

    return rows


if __name__ == "__main__":
    import sys
    main(sys.argv)

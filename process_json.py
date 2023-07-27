#!/usr/bin/env python3
from datetime import timedelta
import json
import math
import os


def main(argv):

    if len(argv) != 2:
        print("Usage: " + argv[0] + " <json folder>")
        sys.exit()
    else:
        jsonDirPath = os.path.abspath(argv[1])

    if not os.path.isdir(jsonDirPath):
        print("Invalid json folder: " + jsonDirPath)
        sys.exit()

    jsonFiles = [file for file in os.listdir(
        jsonDirPath) if file.endswith("json")]

    if len(jsonFiles) == 0:
        print("No json files found in " + jsonDirPath)
        sys.exit()

    benchResults = list()

    for jsonFile in jsonFiles:
        fileName = os.path.basename(jsonFile)
        attributes = parseFilename(splitFilename(fileName))
        jsonFilePath = os.path.join(jsonDirPath, jsonFile)
        with open(jsonFilePath, "r") as f:
            data = json.load(f)
            metrics = parseData(data)

        benchResults.append(metrics | attributes)

    relativePath, _ = os.path.split(os.path.realpath(__file__))
    outputPath = os.path.join(relativePath, "results.json")
    with open(outputPath, "w") as f:
        json.dump(benchResults, f)


def formatSize(num, suffix="B"):
    for unit in ["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"]:
        if abs(num) < 1024.0:
            return f"{num:3.1f}{unit}{suffix}"
        num /= 1024.0
    return f"{num:.1f}Yi{suffix}"


def separateMemory(unit):
    return unit[0:-3], unit[-3:]


def formatTime(delta, pattern):
    d = {'d': delta.days}
    d['h'], rem = divmod(delta.seconds, 3600)
    d['m'], d['s'] = divmod(rem, 60)
    d['u'] = math.ceil(delta.microseconds * (10**3) / 10**6)
    return pattern.format(**d)


def splitFilename(filename):
    name = filename.split("_")
    name[-1] = name[-1].split(".")[0]
    return name


def parseFilename(nameList):
    nameDict = {}
    # print(nameList)
    match nameList[0]:
        case "rust":
            nameDict = {
                "language": nameList[0],
                "benchmark": nameList[1],
                "method": nameList[2],
            }
            if nameList[3] == "async":
                nameDict["library"] = "_".join(nameList[3:5])
                nameDict["threadCount"] = int(nameList[5])
            else:
                nameDict["library"] = nameList[3]
                nameDict["threadCount"] = int(nameList[4])
        case "go":
            nameDict = {
                "language": nameList[0],
                "benchmark": nameList[1],
                "threadCount": int(nameList[2]),
            }
        case "go" | "net6" | "net7":
            nameDict = {
                "language": nameList[0],
                "benchmark": nameList[1],
                "threadCount": int(nameList[2]),
            }
        case "java":
            nameDict = {
                "language": nameList[0],
                "benchmark": nameList[1],
                "method": nameList[2],
                "threadCount": int(nameList[3]),
            }
    return nameDict


def parseData(data):
    realTime = []
    sysTime = []
    userTime = []
    kb = []

    for run in data:
        realTime.append(timedelta(milliseconds=run["real_time_seconds"]*1000))
        sysTime.append(timedelta(milliseconds=run["system_time_seconds"]*1000))
        userTime.append(timedelta(milliseconds=run["user_time_seconds"]*1000))
        kb.append(int(run["max_resident_set_kb"]))

    meanRealTime = sum(realTime, timedelta(0)) / len(realTime)
    meanSysTime = sum(sysTime, timedelta(0)) / len(sysTime)
    meanUserTime = sum(userTime, timedelta(0)) / len(userTime)
    meanKb = sum(kb) / len(kb)
    memoryStr = formatSize(meanKb * 1000)
    memory, memoryUnit = separateMemory(memoryStr)

    metrics = {
        "realTime": float(meanRealTime / timedelta(seconds=1)),
        "cpuTime": float(formatTime(meanSysTime + meanUserTime, "{s}.{u}")),
        "memory": float(memory),
        "memoryUnit": memoryUnit
    }
    return metrics


if __name__ == "__main__":
    import sys
    main(sys.argv)

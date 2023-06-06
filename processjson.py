from datetime import timedelta
import json


def sizeof_fmt(num, suffix="B"):
    for unit in ["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"]:
        if abs(num) < 1024.0:
            return f"{num:3.1f}{unit}{suffix}"
        num /= 1024.0
    return f"{num:.1f}Yi{suffix}"


def main(argv):

    filename = argv[1]
    with open(filename, "r") as f:
        data = json.load(f)

    realTime = []
    kb = []
    sysTime = []
    userTime = []
    cpu = []

    for run in data:
        realTime.append(timedelta(milliseconds=run["real_time_seconds"]*1000))
        kb.append(int(run["max_resident_set_kb"]))
        sysTime.append(timedelta(milliseconds=run["system_time_seconds"]*1000))
        userTime.append(timedelta(milliseconds=run["user_time_seconds"]*1000))
        cpu.append(int(run["cpu_percentage"][:-1]))

    meanRealTime = sum(realTime, timedelta(0)) / len(realTime)
    meanSysTime = sum(sysTime, timedelta(0)) / len(sysTime)
    meanUserTime = sum(userTime, timedelta(0)) / len(userTime)
    meanCpu = sum(cpu) / len(cpu)
    meanKb = sum(kb) / len(kb)

    print(filename)
    print("Real Time: ", meanRealTime)
    print("System Time: ", meanSysTime)
    print("User Time: ", meanUserTime)
    print("CPU: ", meanCpu, "%")
    print("Memory: ", sizeof_fmt(meanKb * 1024))
    print("--------------------")
    print("")


if __name__ == "__main__":
    import sys
    main(sys.argv)

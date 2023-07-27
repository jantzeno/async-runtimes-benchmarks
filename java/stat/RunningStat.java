import java.util.concurrent.locks.ReentrantLock;

class RunningStat {
    private int count = 0;
    private double oldMean = 0;
    private double newMean = 0;
    private double oldSum = 0;
    private double newSum = 0;

    private final ReentrantLock mutex = new ReentrantLock();

    public void RunningStat() {
        this.Clear();
    }

    public void Clear() {
        count = 0;
        oldMean = 0;
        newMean = 0;
        oldSum = 0;
        newSum = 0;
    }

    public int DataValueCount() {
        try {
            mutex.lock();
            return count;
        } finally {
            mutex.unlock();
        }
    }

    public double Mean() {
        try {
            mutex.lock();
            double mean = this.count > 0 ? this.newMean : 0;
            return mean;
        } finally {
            mutex.unlock();
        }
    }

    public double Variance() {
        try {
            mutex.lock();
            double variance = this.count > 1 ? this.newSum / (this.count - 1) : 0;
            return variance;
        } finally {
            mutex.unlock();
        }
    }

    public double StandardDeviation() {
        try {
            mutex.lock();
            double variance = this.Variance();
            return Math.sqrt(variance);
        } finally {
            mutex.unlock();
        }
    }

    public void Push(double value) {
        try {
            mutex.lock();
            this.count++;

            if (this.count == 1) {
                this.oldMean = value;
                this.newMean = value;
                oldSum = 0;
            } else {
                newMean = oldMean + (value - oldMean) / this.count;
                newSum = oldSum + (value - oldMean) * (value - newMean);
            }

            oldMean = newMean;
            oldSum = newSum;

        } finally {
            mutex.unlock();
        }

    }
}
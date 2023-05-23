namespace Stat;

public class RunningStat
{
    int count;
    double old_mean;
    double new_mean;
    double old_sum;
    double new_sum;

    public RunningStat()
    {
        this.Clear();
    }

    public Task Clear()
    {
        this.count = 0;
        this.old_mean = 0;
        this.new_mean = 0;
        this.old_sum = 0;
        this.new_sum = 0;

        return Task.CompletedTask;
    }

    public Task<int> DataValueCount()
    {
        return Task<int>.FromResult(this.count);
    }

    public Task<double> Mean()
    {
        double mean = this.count > 0 ? this.new_mean : 0.0;
        return Task<double>.FromResult(mean);
    }

    public Task<double> Variance()
    {
        double variance = this.count > 1 ? this.new_sum / (this.count - 1) : 0;
        return Task<double>.FromResult(variance);
    }

    public async Task<double> StandardDeviation()
    {
        double variance = await this.Variance();
        return Math.Sqrt(variance);
    }

    public Task Push(double value)
    {
        count++;

        if (count == 1)
        {
            old_mean = value;
            new_mean = value;
            old_sum = 0.0;
        }
        else
        {
            new_mean = old_mean + (value - old_mean) / count;
            new_sum = old_sum + (value - old_mean) * (value - new_mean);
        }

        // set up for next iteration
        old_mean = new_mean;
        old_sum = new_sum;

        return Task.CompletedTask;
    }
}
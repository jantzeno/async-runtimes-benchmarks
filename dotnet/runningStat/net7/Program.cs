using Stat;

public static class Program
{

    static async Task Main(string[] args)
    {
        int numTasks = args.Length > 0 ? int.Parse(args[0]) : 100000;

        double maxTemp = 100.0;
        double minTemp = 0.0;

        RunningStat stat = new RunningStat();

        Console.WriteLine($"Starting {numTasks} tasks");

        List<Task> tasks = new List<Task>();

        for (int i = 0; i < numTasks; i++)
        {
            double temp = NextDouble(new Random(), minTemp, maxTemp);
            Task task = Task.Run(async () =>
            {
                await stat.Push(temp);
            });

            tasks.Add(task);
        }

        await Task.WhenAll(tasks);

        Console.WriteLine("All tasks complete");

        Console.WriteLine($"Mean: {await stat.Mean()}");
        Console.WriteLine($"Count: {await stat.DataValueCount()}");
    }

    static double NextDouble(this Random rng, double min, double max)
    {
        return rng.NextDouble() * (max - min) + min;
    }
}





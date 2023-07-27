import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class VirtualThreads {

    public static void main(String[] args) throws InterruptedException {
        int numTasks = args.length > 0 ? Integer.parseInt(args[0]) : 100000;

        double max = 100;
        double min = 0.0;

        RunningStat stat = new RunningStat();

        System.out.println("Starting " + numTasks + " threads");

        List<Thread> tasks = new ArrayList<>();

        for (int i = 0; i < numTasks; i++) {
            double value = min + new Random().nextDouble() * (max - min);
            Thread thread = Thread.ofVirtual().start(() -> {
                stat.Push(value);
            });
            tasks.add(thread);
        }

        // Wait for all threads to complete
        for (Thread thread : tasks) {
            thread.join();
        }
        System.out.println("All tasks complete");
        System.out.println("Mean: " + stat.Mean());
        System.out.println("Count: " + stat.DataValueCount());
    }
}

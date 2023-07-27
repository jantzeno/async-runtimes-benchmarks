import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.UUID;
import java.util.concurrent.locks.ReentrantLock;

public class Threads {

    public static void main(String[] args) throws InterruptedException {
        int numTasks = args.length > 0 ? Integer.parseInt(args[0]) : 100000;

        ReentrantLock mutex = new ReentrantLock();

        System.out.println("Starting " + numTasks + " threads");

        List<Thread> tasks = new ArrayList<>();
        List<String> hashes = new ArrayList<>();

        for (int i = 0; i < numTasks; i++) {
            Thread thread = new Thread(() -> {
                try {
                    mutex.lock();
                    String uuid = UUID.randomUUID().toString();
                    MessageDigest digest = MessageDigest.getInstance("SHA-512");
                    digest.update(uuid.getBytes(StandardCharsets.UTF_8));
                    byte[] encodedhash = digest.digest();
                    StringBuffer hashCodeBuffer = new StringBuffer();
                    for (int j = 0; j < encodedhash.length; j++) {
                        hashCodeBuffer.append(Integer.toString((encodedhash[j] & 0xff) + 0x100, 16).substring(1));
                    }
                    hashes.add(hashCodeBuffer.toString());

                } catch (NoSuchAlgorithmException e) {
                    e.printStackTrace();
                } finally {
                    mutex.unlock();
                }
            });
            thread.start();
            tasks.add(thread);
        }

        // Wait for all threads to complete
        for (Thread thread : tasks) {
            thread.join();
        }
        System.out.println("All tasks complete");
        System.out.println("Count: " + hashes.size());
    }
}

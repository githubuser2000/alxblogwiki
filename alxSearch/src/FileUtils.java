import java.io.*;
import java.nio.file.*;
import java.util.*;
public class FileUtils {
    public static List<Path> listTextFiles(Path dir, long maxSize) throws IOException {
        List<Path> files = new ArrayList<>();
        Files.walk(dir).forEach(p -> {
            try {
                if(Files.isRegularFile(p) && Files.size(p) <= maxSize && isTextFile(p)) {
                    files.add(p);
                }
            } catch(IOException e) {}
        });
        return files;
    }

    private static boolean isTextFile(Path file) throws IOException {
        try(InputStream in = Files.newInputStream(file)) {
            int b;
            while((b=in.read())!=-1){
                if(b==0) return false; // Nullbyte = Binärdatei
            }
        }
        return true;
    }
}

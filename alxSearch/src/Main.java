import java.nio.file.*;
import java.io.*;
import java.util.*;

public class Main {
    public static void main(String[] args) throws IOException {
        if(args.length<1){
            System.out.println("Usage: java Main <directory>");
            return;
        }
        Path dir = Paths.get(args[0]);
        TokenExtractor extractor = new TokenExtractor();
        List<Path> files = FileUtils.listTextFiles(dir, 500*1024);
        for(Path f : files) extractor.processFile(f);
        extractor.writeCSV("worttokens.csv");
        System.out.println("CSV erzeugt: worttokens.csv");
    }
}

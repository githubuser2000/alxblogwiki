import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.regex.*;

public class WordTokenExtractor {
    public static void main(String[] args) throws IOException {
        if (args.length < 1) {
            System.out.println("Usage: java WordTokenExtractor <directory>");
            return;
        }

        Path dir = Paths.get(args[0]);
        Pattern tokenPattern = Pattern.compile("[A-Z][a-z]*");

        Map<String, Integer> globalCount = new HashMap<>();
        List<String[]> csvRows = new ArrayList<>();

        Files.walk(dir)
            .filter(Files::isRegularFile)
            .filter(f -> {
                try {
                    return Files.size(f) <= 500 * 1024;
                } catch (IOException e) {
                    return false;
                }
            })
            .forEach(file -> {
                Map<String, Integer> fileCount = new HashMap<>();
                try (BufferedReader br = Files.newBufferedReader(file)) {
                    String line;
                    while ((line = br.readLine()) != null) {
                        Matcher m = tokenPattern.matcher(line);
                        while (m.find()) {
                            String token = m.group();
                            fileCount.put(token, fileCount.getOrDefault(token, 0) + 1);
                            globalCount.put(token, globalCount.getOrDefault(token, 0) + 1);
                        }
                    }
                    for (Map.Entry<String, Integer> entry : fileCount.entrySet()) {
                        csvRows.add(new String[]{entry.getKey(), file.getFileName().toString(), "0", entry.getValue().toString()});
                    }
                } catch (IOException e) { e.printStackTrace(); }
            });

        // Update global count
        for (String[] row : csvRows) {
            row[2] = globalCount.get(row[0]).toString();
        }

        try (PrintWriter pw = new PrintWriter("worttokens.csv")) {
            pw.println("Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei");
            for (String[] row : csvRows) {
                pw.println(String.join(",", row));
            }
        }

        System.out.println("CSV erzeugt: worttokens.csv");
    }
}

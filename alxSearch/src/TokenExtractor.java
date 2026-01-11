import java.io.*;
import java.nio.file.Path;
import java.util.*;
import java.util.regex.*;

public class TokenExtractor {
    private Map<String,Integer> globalCount = new HashMap<>();
    private List<String[]> rows = new ArrayList<>();
    private Pattern tokenPattern = Pattern.compile("[A-Z][a-z]*");

    public void processFile(Path file) throws IOException {
        Map<String,Integer> fileCount = new HashMap<>();
        try(BufferedReader br = Files.newBufferedReader(file)){
            String line;
            while((line=br.readLine())!=null){
                Matcher m = tokenPattern.matcher(line);
                while(m.find()){
                    String token = m.group();
                    fileCount.put(token, fileCount.getOrDefault(token,0)+1);
                    globalCount.put(token, globalCount.getOrDefault(token,0)+1);
                }
            }
        }
        for(Map.Entry<String,Integer> e : fileCount.entrySet()){
            rows.add(new String[]{e.getKey(), file.getFileName().toString(), "0", e.getValue().toString()});
        }
    }

    public void writeCSV(String csvFile) throws IOException {
        try(PrintWriter pw = new PrintWriter(csvFile)){
            pw.println("Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei");
            for(String[] row : rows){
                row[2] = globalCount.get(row[0]).toString();
                pw.println(String.join(",", row));
            }
        }
    }
}

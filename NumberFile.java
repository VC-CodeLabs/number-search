import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.FileNotFoundException;

public class NumberFile {
    public static void main(String[] args) {
        System.out.println("In here!");
        try {
            FileReader myReader = new FileReader("inputFile.txt");

            try {
                FileWriter myWriter = new FileWriter("output.txt");
                boolean newLine = true;

                int i;
                char firstNumeral = 0, lastNumeral = 0, lastChar = 0; // Note, these are chars, not ints;
                while ((i = myReader.read()) != -1) { // read every char in order

                    if ((i >= 48 && i <= 57)) { // the char in question is a numeral
                        if (newLine) {
                            firstNumeral = (char) i;
                            newLine = false; // after this, only update lastNumeral;
                        }
                        lastNumeral = (char) i;
                    } else if ((char) i == '\n') { // new line - write only the first and last numerals to the
                                                   // outputFile.txt
                        newLine = true;

                        if (firstNumeral != 0 && lastNumeral != 0) {
                            // System.out.println(firstNumeral + " " + lastNumeral + " ");
                            myWriter.write(firstNumeral);
                            myWriter.write(lastNumeral);
                            myWriter.write('\n');
                            firstNumeral = 0; // resetting the variables to null
                            lastNumeral = 0; // resetting the variables to null
                        }
                    }
                    lastChar = (char) i;
                }
                // System.out.println(firstNumeral + " " + lastNumeral + " ");
                /*
                 * if (lastChar != '\n') { Originally I thought I'd need this if statement,
                 * since, were there a \n, I wouldn't want to print it twice
                 * However, since I added the firstNumeral and lastNumeral resets, I realize I
                 * should print two nulls if there is a final line with nothing on it.
                 */
                myWriter.write(firstNumeral); // Because there is no \n on the last line, I need to print out the last
                                              // one.
                myWriter.write(lastNumeral);
                // }

                myWriter.close();
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        } catch (FileNotFoundException e) {
            System.out.println("File not found.");
            e.printStackTrace();
        }
    }
}
